use advent2020::errors::{MapOperationError, MapParseError, TopLevelError};
use advent2020::map::Map;
use std::convert::TryFrom;
use std::env;
use std::fs;

#[derive(Clone, Debug, Eq, PartialEq)]
enum FerryLocation {
    Floor,
    EmptySeat,
    TakenSeat,
}

impl FerryLocation {
    fn is_seat(&self) -> bool {
        self != &FerryLocation::Floor
    }
}

impl TryFrom<char> for FerryLocation {
    type Error = MapParseError;

    fn try_from(c: char) -> Result<Self,Self::Error> {
        match c {
            '.' => Ok(FerryLocation::Floor),
            'L' => Ok(FerryLocation::EmptySeat),
            '#' => Ok(FerryLocation::TakenSeat),
            _ => Err(MapParseError::UnexpectedCharacter(c)),
        }
    }
}

impl Into<char> for FerryLocation {
    fn into(self) -> char {
        match self {
            FerryLocation::Floor => '.',
            FerryLocation::EmptySeat => 'L',
            FerryLocation::TakenSeat => '#',
        }
    }
}

struct EvolvingMap {
    next_map: Option<Map<FerryLocation>>,
    occupation_tolerance: usize,
    view: fn(&Map<FerryLocation>, usize, usize) -> Result<Vec<FerryLocation>, MapOperationError>,
}

impl From<Map<FerryLocation>> for EvolvingMap {
    fn from(start_map: Map<FerryLocation>) -> EvolvingMap {
        EvolvingMap {
            next_map: Some(start_map),
            occupation_tolerance: 4,
            view: |m, x, y| { m.adjacents(x, y) },
         }
    }
}

impl Iterator for EvolvingMap {
    type Item = Map<FerryLocation>;

    fn next(&mut self) -> Option<Self::Item> {
        let current_map = self.next_map.clone()?;
        let mut next_map = current_map.clone();

        for (x, y, value) in current_map.locations() {
            let occupied_neighbors = (self.view)(&current_map, x, y).ok()?.iter().filter(|x| **x == FerryLocation::TakenSeat).count();
            match value {
                FerryLocation::EmptySeat if occupied_neighbors == 0 => {
                    next_map.set(x, y, FerryLocation::TakenSeat).ok()?;
                }
                FerryLocation::TakenSeat if occupied_neighbors >= self.occupation_tolerance => {
                    next_map.set(x, y, FerryLocation::EmptySeat).ok()?;
                }
                _ => {}
            }
        }

        if next_map == current_map {
            self.next_map = None;
        } else {
            self.next_map = Some(next_map);
        }

        Some(current_map)
    }
}

fn main() -> Result<(),TopLevelError> {
    let filename = env::args().nth(1).expect("No file argument given.");
    let contents = fs::read_to_string(filename)?;
    let map = Map::<FerryLocation>::try_from(contents.as_str())?;

    // part 1
    let base_evolving_map = EvolvingMap::from(map.clone());
    for (idx, step) in base_evolving_map.enumerate() {
        println!("Base map, step #{}", idx);
        step.print();
        println!("# of occupied seats: {}\n", step.count(FerryLocation::TakenSeat));
    }

    // part 2
    let view_evolving_map = EvolvingMap {
        next_map: Some(map),
        occupation_tolerance: 5,
        view: |m,x,y| m.adjacents_until(x, y, FerryLocation::is_seat),
    };
    for (idx, step) in view_evolving_map.enumerate() {
        println!("Extended map, step #{}", idx);
        step.print();
        println!("# of occupied seats: {}\n", step.count(FerryLocation::TakenSeat));
    }

    Ok(())
}