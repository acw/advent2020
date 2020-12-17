use advent2020::errors::{TicketParseError, TopLevelError};
use std::env;
use std::fs;
use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(Clone, Debug)]
struct Field<'a> {
    name: &'a str,
    ranges: Vec<RangeInclusive<usize>>,
}

impl<'a> PartialEq for Field<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl<'a> Field<'a> {
    // interestingly, this cannot be FromStr, because the lifetimes can't be worked
    // out with that trait.
    fn new(s: &'a str) -> Result<Field<'a>, TicketParseError> {
        let splits: Vec<&str> = s.split(": ").collect();

        if splits.len() != 2 {
            return Err(TicketParseError::BadFieldDefinition(s.to_string()));
        }
        let mut ranges = Vec::new();
        for range_str in splits[1].split(" or ") {
            let numbers: Vec<&str> = range_str.split('-').collect();
            if numbers.len() != 2 {
                return Err(TicketParseError::BadFieldDefinition(range_str.to_string()));
            }
            let left = usize::from_str(numbers[0])?;
            let right = usize::from_str(numbers[1])?;
            ranges.push(left..=right);
        }

        Ok(Field {
            name: splits[0],
            ranges,
        })
    }

    fn ok_with(&self, value: usize) -> bool {
        self.ranges.iter().any(|x| x.contains(&value))
    }
}

fn parse_fields<'a, I: Iterator<Item = &'a str>>(
    iterator: &mut I,
) -> Result<Vec<Field<'a>>, TicketParseError> {
    let mut res = Vec::new();

    for x in iterator {
        if x == "" {
            return Ok(res);
        } else {
            res.push(Field::new(x)?)
        }
    }

    Err(TicketParseError::UnterminatedFieldDefs)
}

#[derive(Clone)]
struct Ticket(Vec<usize>);

impl Ticket {
    fn new(s: &str) -> Result<Ticket, TicketParseError> {
        let mut results = Vec::new();

        for value_str in s.split(',') {
            results.push(usize::from_str(value_str)?);
        }

        Ok(Ticket(results))
    }

    fn invalid_field(&self, fields: &[Field]) -> Option<usize> {
        for x in self.0.iter() {
            if !fields.iter().any(|f| f.ok_with(*x)) {
                println!("{} cannot be valid", x);
                return Some(*x);
            }
        }
        None
    }
}

fn parse_my_ticket<'a, I: Iterator<Item = &'a str>>(
    iterator: &mut I,
) -> Result<Ticket, TicketParseError> {
    let key = iterator
        .next()
        .ok_or(TicketParseError::YourTicketParseError)?;
    if key != "your ticket:" {
        return Err(TicketParseError::YourTicketParseError);
    }

    let values = iterator
        .next()
        .ok_or(TicketParseError::YourTicketParseError)?;
    let ticket = Ticket::new(values)?;

    let suffix = iterator
        .next()
        .ok_or(TicketParseError::YourTicketParseError)?;
    if suffix != "" {
        return Err(TicketParseError::YourTicketParseError);
    }

    Ok(ticket)
}

fn parse_nearby_tickets<'a, I: Iterator<Item = &'a str>>(
    iterator: &mut I,
) -> Result<Vec<Ticket>, TicketParseError> {
    let key = iterator
        .next()
        .ok_or(TicketParseError::YourTicketParseError)?;
    if key != "nearby tickets:" {
        return Err(TicketParseError::YourTicketParseError);
    }

    let mut results = Vec::new();
    for line in iterator {
        results.push(Ticket::new(line)?);
    }

    Ok(results)
}

fn resolve_theories(mut theories: Vec<Vec<Field>>) -> Result<Vec<Field>, TopLevelError> {
    let mut changed_something = true;

    while changed_something && theories.iter().any(|x| x.len() > 1) {
        changed_something = false;

        if theories.iter().any(|x| x.is_empty()) {
            return Err(TopLevelError::NoSolutionFound);
        }

        let singletons: Vec<Field> = theories
            .iter()
            .filter_map(|x| {
                if x.len() == 1 {
                    Some(x[0].clone())
                } else {
                    None
                }
            })
            .collect();

        for theory in theories.iter_mut() {
            if theory.len() > 1 {
                let old_len = theory.len();
                theory.retain(|v| !singletons.contains(&v));
                changed_something |= old_len != theory.len();
            }
        }
    }

    if changed_something {
        Ok(theories.drain(..).map(|mut x| x.pop().unwrap()).collect())
    } else {
        Err(TopLevelError::NoInputFound)
    }
}

fn main() -> Result<(), TopLevelError> {
    let filename = env::args().nth(1).expect("No file argument given.");
    let contents = fs::read_to_string(filename)?;
    let mut lines = contents.lines();

    let fields = parse_fields(&mut lines)?;
    let my_ticket = parse_my_ticket(&mut lines)?;
    let nearby_tickets = parse_nearby_tickets(&mut lines)?;
    let mut valid_tickets = Vec::new();
    let mut sum = 0;

    for ticket in nearby_tickets {
        if let Some(x) = ticket.invalid_field(&fields) {
            sum += x;
        } else {
            valid_tickets.push(ticket);
        }
    }

    println!("Ticket scanning error rate: {}", sum);

    let mut theories = Vec::with_capacity((&my_ticket.0).len());
    theories.resize(my_ticket.0.len(), fields);
    valid_tickets.push(my_ticket.clone());
    println!("There are {} valid tickets", valid_tickets.len());

    for valid_ticket in valid_tickets.iter() {
        for (ticket_field, theories) in valid_ticket.0.iter().zip(theories.iter_mut()) {
            theories.retain(|f| f.ok_with(*ticket_field));
        }
    }

    let resolved_theories = resolve_theories(theories)?;
    assert_eq!(resolved_theories.len(), (&my_ticket.0).len());

    let mut departure_product = 1;

    for (field_info, value) in resolved_theories.iter().zip(my_ticket.0.iter()) {
        println!("{}: {}", field_info.name, value);
        if field_info.name.starts_with("departure") {
            departure_product *= value;
        }
    }
    println!("Departure product: {}", departure_product);

    Ok(())
}
