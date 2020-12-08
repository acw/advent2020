use advent2020::errors::{BaggageRuleParseError, TopLevelError};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, char, digit1, multispace0, multispace1};
use nom::multi::{fold_many1, separated_list1};
use nom::sequence::preceded;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::str::FromStr;

struct RuleSet {
    contain_rules: HashMap<String, Vec<Rule>>,
}

#[derive(Clone)]
struct Rule {
    count: usize,
    bag: String,
}

impl RuleSet {
    fn empty() -> RuleSet {
        RuleSet {
            contain_rules: HashMap::new(),
        }
    }

    fn merge(&mut self, other: &mut RuleSet) {
        self.contain_rules.extend(other.contain_rules.drain());
    }

    fn pretty_print(&self) {
        for (key, value) in self.contain_rules.iter() {
            if value.is_empty() {
                println!("{} --> <empty>", key);
            } else {
                let blank = " ".repeat(key.len());
                let mut first = true;

                for rule in value.iter() {
                    println!(
                        "{} --> {} {}",
                        if first { key } else { &blank },
                        rule.count,
                        rule.bag
                    );
                    first = false;
                }
            }
        }
    }

    fn can_reach(&self, start: &str, end: &str) -> bool {
        let mut stack = vec![start];
        let mut visited = HashSet::new();

        while let Some(next) = stack.pop() {
            if next == end {
                return true;
            }

            if visited.contains(next) {
                continue;
            }

            visited.insert(next);

            match self.contain_rules.get(next) {
                None => println!("WARNING: Can't find color {}", next),
                Some(rules) => {
                    for rule in rules.iter() {
                        stack.push(&rule.bag);
                    }
                }
            }
        }

        false
    }

    fn bags_required(&self, color: &str) -> usize {
        match self.contain_rules.get(color) {
            None => {
                println!("WARNING: Can't find color {}", color);
                0
            }
            Some(rules) => {
                rules
                    .iter()
                    .map(|x| self.bags_required(&x.bag) * x.count)
                    .sum::<usize>()
                    + 1
            }
        }
    }
}

impl FromStr for RuleSet {
    type Err = BaggageRuleParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, parse_result) = parse_rules(s)?;
        Ok(RuleSet {
            contain_rules: parse_result,
        })
    }
}

fn parse_rules(input0: &str) -> nom::IResult<&str, HashMap<String, Vec<Rule>>> {
    fold_many1(parse_rule, HashMap::new(), |mut acc, (key, value)| {
        acc.insert(key, value);
        acc
    })(input0)
}

fn parse_rule(input0: &str) -> nom::IResult<&str, (String, Vec<Rule>)> {
    let (input1, _) = multispace0(input0)?;
    let (input2, key_color) = parse_color(input1)?;
    let (input3, _) = multispace1(input2)?;
    let (input4, _) = tag("bags")(input3)?;
    let (input5, _) = multispace1(input4)?;
    let (input6, _) = tag("contain")(input5)?;
    let (input7, _) = multispace1(input6)?;
    let (input8, rules) = parse_bag_set(input7)?;
    let (input9, _) = multispace0(input8)?;
    let (input10, _) = tag(".")(input9)?;

    Ok((input10, (key_color, rules)))
}

fn parse_color(input0: &str) -> nom::IResult<&str, String> {
    let (input1, _) = multispace0(input0)?;
    let (input2, word1) = alphanumeric1(input1)?;
    let (input3, _) = multispace1(input2)?;
    let (input4, word2) = alphanumeric1(input3)?;

    Ok((input4, format!("{} {}", word1, word2)))
}

fn parse_bag_set(input0: &str) -> nom::IResult<&str, Vec<Rule>> {
    let (input1, _) = multispace0(input0)?;
    let (input2, list) = alt((parse_no_rules, parse_rule_list))(input1)?;

    Ok((input2, list))
}

fn parse_no_rules(input0: &str) -> nom::IResult<&str, Vec<Rule>> {
    let (input1, _) = multispace0(input0)?;
    let (input2, _) = tag("no")(input1)?;
    let (input3, _) = multispace1(input2)?;
    let (input4, _) = tag("other")(input3)?;
    let (input5, _) = multispace1(input4)?;
    let (input6, _) = tag("bags")(input5)?;

    Ok((input6, Vec::new()))
}

fn parse_rule_list(input0: &str) -> nom::IResult<&str, Vec<Rule>> {
    let (input1, _) = multispace0(input0)?;
    let (input2, list) =
        separated_list1(preceded(char(','), multispace1), parse_rule_item)(input1)?;

    Ok((input2, list))
}

fn parse_rule_item(input0: &str) -> nom::IResult<&str, Rule> {
    let (input1, _) = multispace0(input0)?;
    let (input2, number_string) = digit1(input1)?;
    let (input3, _) = multispace1(input2)?;
    let (input4, bag) = parse_color(input3)?;
    let (input5, _) = multispace1(input4)?;

    let count = usize::from_str(number_string).map_err(|_| {
        nom::Err::Error(nom::error::Error {
            input: input2,
            code: nom::error::ErrorKind::Digit,
        })
    })?;
    let (input6, _) = if count == 1 {
        tag("bag")(input5)?
    } else {
        tag("bags")(input5)?
    };

    Ok((input6, Rule { count, bag }))
}

fn main() -> Result<(), TopLevelError> {
    let mut rules = RuleSet::empty();

    for argument in env::args().skip(1) {
        let contents = fs::read_to_string(argument)?;
        let mut this_one = RuleSet::from_str(&contents)?;
        rules.merge(&mut this_one);
    }
    rules.pretty_print();

    let mut count = 0;

    for color in rules.contain_rules.keys() {
        if color != "shiny gold" && rules.can_reach(color, "shiny gold") {
            println!("I can get to shiny gold from {}", color);
            count += 1;
        }
    }
    println!("Can reach shiny gold from {} colors.", count);
    println!(
        "A single shiny gold bag contains {} colors.",
        rules.bags_required("shiny gold") - 1
    );

    Ok(())
}