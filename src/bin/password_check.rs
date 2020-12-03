use advent2020::errors::{PasswordParseError, TopLevelError};
use nom::character::complete::{anychar, char, digit1, multispace1};
use std::env;
use std::fs;
use std::str::FromStr;

struct PasswordData {
    first_number: usize,
    second_number: usize,
    character: char,
    password: String,
}

impl FromStr for PasswordData {
    type Err = PasswordParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (rest0, min_chars) = digit1(s)?;
        let (rest1, _) = char('-')(rest0)?;
        let (rest2, max_chars) = digit1(rest1)?;
        let (rest3, _) = multispace1(rest2)?;
        let (rest4, character) = anychar(rest3)?;
        let (rest5, _) = char(':')(rest4)?;
        let (password_chars, _) = multispace1(rest5)?;

        let min_count = usize::from_str(min_chars)?;
        let max_count = usize::from_str(max_chars)?;
        let password = password_chars.to_string();

        Ok(PasswordData {
            first_number: min_count,
            second_number: max_count,
            character,
            password,
        })
    }
}

impl PasswordData {
    fn is_valid_interpretation1(&self) -> bool {
        let count = self
            .password
            .chars()
            .filter(|x| *x == self.character)
            .count();
        (count >= self.first_number) && (count <= self.second_number)
    }

    fn is_valid_interpretation2(&self) -> bool {
        let mut first_matches = false;
        let mut second_matches = false;

        for (idx, char) in self.password.char_indices() {
            if (idx + 1) == self.first_number {
                first_matches = char == self.character;
            }

            if (idx + 1) == self.second_number {
                second_matches = char == self.character;
            }
        }

        first_matches ^ second_matches
    }
}

fn real_main() -> Result<(), TopLevelError> {
    let mut good_items_interpretation1 = 0u64;
    let mut good_items_interpretation2 = 0u64;

    for argument in env::args().skip(1) {
        println!("Processing {}", argument);
        let contents = fs::read_to_string(argument)?;
        for line in contents.lines() {
            match PasswordData::from_str(line) {
                Err(e) => eprintln!("Skipping line with '{}': {}", line, e),
                Ok(v) => {
                    if v.is_valid_interpretation1() {
                        good_items_interpretation1 += 1;
                    }
                    if v.is_valid_interpretation2() {
                        good_items_interpretation2 += 1;
                    }
                }
            }
        }
    }

    println!(
        "# of good passwords, according to interpretation #1: {}",
        good_items_interpretation1
    );
    println!(
        "# of good passwords, according to interpretation #2: {}",
        good_items_interpretation2
    );

    Ok(())
}

fn main() {
    match real_main() {
        Err(e) => eprintln!("ERROR: {}", e),
        Ok(_) => {}
    }
}
