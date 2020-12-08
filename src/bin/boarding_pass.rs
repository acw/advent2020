use advent2020::errors::{SeatParseError, TopLevelError};
use std::cmp::{Ord, Ordering};
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::str::FromStr;

const PLANE_ROWS: usize = 128;
const PLANE_COLUMNS: usize = 8;

#[derive(Debug)]
struct Seat {
    row: usize,
    column: usize,
    id: usize,
}

impl PartialOrd for Seat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Seat {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Seat {}

impl Ord for Seat {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl FromStr for Seat {
    type Err = SeatParseError;

    fn from_str(map: &str) -> Result<Self, Self::Err> {
        if map.len() != 10 {
            return Err(SeatParseError::InvalidSeatIdentifier(map.to_string()));
        }

        let (row_stuff, column_stuff) = map.split_at(7);

        let mut row_high = PLANE_ROWS - 1;
        let mut row_low = 0;

        for direction in row_stuff.chars() {
            match direction {
                'F' => row_high = row_low + ((row_high - row_low) / 2),
                'B' => row_low = row_low + ((row_high - row_low + 1) / 2),
                _ => return Err(SeatParseError::UnexpectedRowCharacter(direction)),
            }
        }

        if row_high != row_low {
            return Err(SeatParseError::DidNotResolveRow(map.to_string()));
        }

        let mut column_high = PLANE_COLUMNS - 1;
        let mut column_low = 0;

        for direction in column_stuff.chars() {
            match direction {
                'L' => column_high = column_low + ((column_high - column_low) / 2),
                'R' => column_low = column_low + ((column_high - column_low + 1) / 2),
                _ => return Err(SeatParseError::UnexpectedColumnCharacter(direction)),
            }
        }

        if column_high != column_low {
            return Err(SeatParseError::DidNotResolveColumn(map.to_string()));
        }

        Ok(Seat {
            row: row_high,
            column: column_high,
            id: (row_high * PLANE_COLUMNS) + column_high,
        })
    }
}

#[test]
fn example_boarding_passes() {
    assert_eq!(
        Seat::from_str("FBFBBFFRLR"),
        Ok(Seat {
            row: 44,
            column: 5,
            id: 357
        })
    );
    assert_eq!(
        Seat::from_str("BFFFBBFRRR"),
        Ok(Seat {
            row: 70,
            column: 7,
            id: 567
        })
    );
    assert_eq!(
        Seat::from_str("FFFBBBFRRR"),
        Ok(Seat {
            row: 14,
            column: 7,
            id: 119
        })
    );
    assert_eq!(
        Seat::from_str("BBFFBBFRLL"),
        Ok(Seat {
            row: 102,
            column: 4,
            id: 820
        })
    );
}

fn real_main() -> Result<(), TopLevelError> {
    let mut seats = BTreeSet::new();
    let mut highest_id = 0;

    for argument in env::args().skip(1) {
        let contents = fs::read_to_string(argument)?;

        for line in contents.lines() {
            let seat = Seat::from_str(line)?;
            if seat.id > highest_id {
                highest_id = seat.id;
            }
            seats.insert(seat);
        }
    }

    println!("Loaded {} seats.", seats.len());
    println!("  highest id is {}", highest_id);
    let mut last_id = 0;
    for seat in seats.iter() {
        if seat.id == last_id + 2 {
            println!("  my seat is {}", last_id + 1);
            return Ok(());
        }
        last_id = seat.id;
    }

    Err(TopLevelError::NoSolutionFound)
}

fn main() {
    if let Err(e) = real_main() {
        eprintln!("ERROR: {}", e);
    }
}
