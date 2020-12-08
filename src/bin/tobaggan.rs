use advent2020::errors::{MapParseError, TopLevelError};
use advent2020::map::Map;
use std::convert::TryFrom;
use std::env;
use std::fs;

#[derive(Clone, Debug)]
enum Square {
    Empty,
    Tree,
}

impl TryFrom<char> for Square {
    type Error = MapParseError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Square::Empty),
            '#' => Ok(Square::Tree),
            _ => Err(MapParseError::UnexpectedCharacter(c)),
        }
    }
}

struct Encounters {
    trees_encountered: usize,
    clear_spots_encountered: usize,
}

fn trail_for_slope(map: &Map<Square>, run: usize, fall: usize) -> Encounters {
    let mut current_x = 0;
    let mut current_y = 0;
    let mut encounters = Encounters {
        trees_encountered: 0,
        clear_spots_encountered: 0,
    };

    loop {
        match map.at(current_x, current_y) {
            None => break,
            Some(Square::Empty) => encounters.clear_spots_encountered += 1,
            Some(Square::Tree) => encounters.trees_encountered += 1,
        }

        current_x += run;
        current_y += fall;
    }

    encounters
}

fn real_main() -> Result<(), TopLevelError> {
    let mut maybe_map = None;

    for argument in env::args().skip(1) {
        let fname = argument.clone();
        let contents = fs::read_to_string(argument)?;
        match Map::<Square>::try_from(contents.as_str()) {
            Err(e) => eprintln!("Skipping file {}: Parse error: {}", fname, e),
            Ok(v) => {
                maybe_map = Some(v);
                break;
            }
        }
    }

    let map = match maybe_map {
        None => return Err(TopLevelError::NoInputFound),
        Some(x) => x,
    };

    let initial = trail_for_slope(&map, 3, 1);
    println!(
        "For the initial slope, encountered {} trees and {} open spaces",
        initial.trees_encountered, initial.clear_spots_encountered
    );

    let mut product = 1;
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    for (run, fall) in slopes.iter() {
        let encounters = trail_for_slope(&map, *run, *fall);
        println!(
            "For slope ({},{}), encountered {} trees and {} open spaces",
            run, fall, encounters.trees_encountered, encounters.clear_spots_encountered
        );
        product *= encounters.trees_encountered;
    }

    println!("The product of the trees encountered is {}", product);

    Ok(())
}

fn main() {
    if let Err(e) = real_main() {
        eprintln!("ERROR: {}", e);
    }
}
