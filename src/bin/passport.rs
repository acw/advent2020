use advent2020::errors::{PassportParseError, TopLevelError};
use std::env;
use std::fs;
use std::str::FromStr;

#[derive(Debug)]
struct Passport {
    birth_year: Option<String>,
    issue_year: Option<String>,
    expiration_year: Option<String>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
}

const VALID_EYE_COLORS: &[&str] = &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

macro_rules! valid_range {
    ($str: expr, $low: expr, $high: expr) => {
        match u64::from_str($str) {
            Err(_) => return false,
            Ok(x) if x < $low || x > $high => return false,
            Ok(_) => {}
        }
    };
}

impl Passport {
    fn new() -> Passport {
        Passport {
            birth_year: None,
            issue_year: None,
            expiration_year: None,
            height: None,
            hair_color: None,
            eye_color: None,
            passport_id: None,
            country_id: None,
        }
    }

    fn injest_data(&mut self, line: &str) -> Result<(), PassportParseError> {
        for item in line.split_whitespace() {
            let parts: Vec<&str> = item.split(':').collect();

            if parts.len() != 2 {
                return Err(PassportParseError::InvalidChunk(item.to_string()));
            }

            match parts[0] {
                "byr" => self.birth_year = Some(parts[1].to_string()),
                "iyr" => self.issue_year = Some(parts[1].to_string()),
                "eyr" => self.expiration_year = Some(parts[1].to_string()),
                "hgt" => self.height = Some(parts[1].to_string()),
                "hcl" => self.hair_color = Some(parts[1].to_string()),
                "ecl" => self.eye_color = Some(parts[1].to_string()),
                "pid" => self.passport_id = Some(parts[1].to_string()),
                "cid" => self.country_id = Some(parts[1].to_string()),
                unknown => return Err(PassportParseError::InvalidField(unknown.to_string())),
            }
        }

        Ok(())
    }

    fn is_basically_valid(&self) -> bool {
        self.birth_year.is_some()
            && self.issue_year.is_some()
            && self.expiration_year.is_some()
            && self.height.is_some()
            && self.hair_color.is_some()
            && self.eye_color.is_some()
            && self.passport_id.is_some()
    }

    fn is_really_valid(&self) -> bool {
        // check the years
        if !valid_year_range(&self.birth_year, 1920, 2002) {
            return false;
        }

        if !valid_year_range(&self.issue_year, 2010, 2020) {
            return false;
        }

        if !valid_year_range(&self.expiration_year, 2020, 2030) {
            return false;
        }

        // check the height
        match self.height {
            None => return false,
            Some(ref x) => {
                if let Some(idx) = x.rfind("cm") {
                    valid_range!(&x[0..idx], 150, 193);
                } else if let Some(idx) = x.rfind("in") {
                    valid_range!(&x[0..idx], 59, 76);
                } else {
                    return false;
                }
            }
        }

        // check the hair color
        match self.hair_color {
            None => return false,
            Some(ref x) if x.len() != 7 => return false,
            Some(ref x) => {
                if !x.starts_with('#') || x[1..].chars().any(|x| !x.is_digit(16)) {
                    return false;
                }
            }
        }

        // check the eye color
        match self.eye_color {
            None => return false,
            Some(ref x) if VALID_EYE_COLORS.contains(&x.as_str()) => {}
            Some(_) => return false,
        }

        // check the passport number
        match self.passport_id {
            None => false,
            Some(ref x) => x.len() == 9 && x.chars().all(|x| x.is_digit(10)),
        }
    }
}

fn valid_year_range(field: &Option<String>, start: u64, end: u64) -> bool {
    match field {
        None => return false,
        Some(x) => valid_range!(x, start, end),
    }
    true
}

fn main() -> Result<(), TopLevelError> {
    let mut passports = Vec::new();

    for argument in env::args().skip(1) {
        let contents = fs::read_to_string(argument)?;
        let mut current_passport = Passport::new();

        for line in contents.lines() {
            if line == "" {
                passports.push(current_passport);
                current_passport = Passport::new();
            } else {
                current_passport.injest_data(line)?;
            }
        }
        passports.push(current_passport);
    }

    let valid_passports: Vec<&Passport> = passports
        .iter()
        .filter(|x| x.is_basically_valid())
        .collect();
    println!(
        "There are {} *basically* valid passports.",
        valid_passports.len()
    );
    println!(
        "   ... {} are really valid",
        passports.iter().filter(|x| x.is_really_valid()).count()
    );

    Ok(())
}
