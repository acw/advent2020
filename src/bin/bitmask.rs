use advent2020::errors::{BitmaskCommandParseError, MaskParseError, TopLevelError};
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::str::FromStr;

struct Mask {
    or_part: u64,
    and_part: u64,
}

impl FromStr for Mask {
    type Err = MaskParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 36 {
            return Err(MaskParseError::WrongLength(s.len()));
        }

        let mut or_part = 0;
        let mut and_part = 0;

        for char in s.chars() {
            or_part <<= 1;
            and_part <<= 1;
            match char {
                'X' => and_part |= 1,
                '0' => {}
                '1' => or_part |= 1,
                _ => return Err(MaskParseError::UnexpectedCharacter(char)),
            }
        }

        Ok(Mask { or_part, and_part })
    }
}

impl Mask {
    fn new() -> Mask {
        Mask {
            or_part: 0,
            and_part: 0b1111_11111111_11111111_11111111_11111111,
        }
    }

    fn mask(&self, value: u64) -> u64 {
        (value & self.and_part) | self.or_part
    }
}

#[test]
fn basic_mask_tests() {
    let test1 = Mask::from_str("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X").unwrap();
    assert_eq!(73, test1.mask(11));
    let test2 = Mask::from_str("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X").unwrap();
    assert_eq!(101, test2.mask(101));
    let test3 = Mask::from_str("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X").unwrap();
    assert_eq!(64, test3.mask(0));
}

enum Command<M> {
    SetMask(M),
    WriteMemory(usize, u64),
}

impl<M, E> FromStr for Command<M>
where
    M: FromStr<Err = E>,
    BitmaskCommandParseError: From<E>,
{
    type Err = BitmaskCommandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" = ");

        match parts.next() {
            None => Err(BitmaskCommandParseError::EmptyCommand),
            Some("mask") => match parts.next() {
                None => Err(BitmaskCommandParseError::PartialCommand("mask".to_string())),
                Some(x) => Ok(Command::SetMask(M::from_str(x)?)),
            },
            Some(left) if left.starts_with("mem[") => match parts.next() {
                None => Err(BitmaskCommandParseError::PartialCommand(left.to_string())),
                Some(value_str) => {
                    let numerics: String = left
                        .chars()
                        .skip(4)
                        .take_while(|x| x.is_digit(10))
                        .collect();
                    let location = usize::from_str(&numerics)?;
                    let value = u64::from_str(value_str)?;
                    Ok(Command::WriteMemory(location, value))
                }
            },
            Some(left) => Err(BitmaskCommandParseError::UnknownCommand(left.to_string())),
        }
    }
}

struct Computer<M> {
    mask: M,
    locations: BTreeMap<usize, u64>,
}

impl Computer<Mask> {
    fn new() -> Computer<Mask> {
        Computer {
            mask: Mask::new(),
            locations: BTreeMap::new(),
        }
    }

    fn step(&mut self, m: Command<Mask>) {
        match m {
            Command::SetMask(new_mask) => self.mask = new_mask,
            Command::WriteMemory(location, value) => {
                let _ = self.locations.insert(location, self.mask.mask(value));
            }
        }
    }
}

struct FloatyMask {
    or_part: usize,
    floating_bits: Vec<usize>,
}

impl FromStr for FloatyMask {
    type Err = MaskParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 36 {
            return Err(MaskParseError::WrongLength(s.len()));
        }

        let mut bitno = 36;
        let mut or_part = 0;
        let mut floating_bits = Vec::new();

        for char in s.chars() {
            or_part <<= 1;
            bitno -= 1;
            match char {
                'X' => floating_bits.push(bitno),
                '0' => {}
                '1' => or_part |= 1,
                _ => return Err(MaskParseError::UnexpectedCharacter(char)),
            }
        }

        Ok(FloatyMask {
            or_part,
            floating_bits,
        })
    }
}

impl FloatyMask {
    fn new() -> FloatyMask {
        FloatyMask {
            or_part: 0,
            floating_bits: vec![],
        }
    }

    fn mask(&self, value: usize) -> Vec<usize> {
        let base = value | self.or_part;
        let mut variants = vec![base];

        for bit in self.floating_bits.iter() {
            let mut variants_zero = variants.clone();

            for value in variants.iter_mut() {
                *value |= 1 << bit;
            }
            for value in variants_zero.iter_mut() {
                *value &= !(1 << bit);
            }

            variants.append(&mut variants_zero);
        }

        variants
    }
}

impl Computer<FloatyMask> {
    fn new() -> Computer<FloatyMask> {
        Computer {
            mask: FloatyMask::new(),
            locations: BTreeMap::new(),
        }
    }

    fn step(&mut self, m: Command<FloatyMask>) {
        match m {
            Command::SetMask(new_mask) => self.mask = new_mask,
            Command::WriteMemory(location, value) => {
                for location in self.mask.mask(location) {
                    self.locations.insert(location, value);
                }
            }
        }
    }
}

#[test]
fn floaty_mask() {
    let test1 = FloatyMask::from_str("000000000000000000000000000000X1001X").unwrap();
    assert_eq!(2, test1.floating_bits.len());
    let result1 = test1.mask(42);
    assert_eq!(4, result1.len());
    assert!(result1.contains(&26));
    assert!(result1.contains(&27));
    assert!(result1.contains(&58));
    assert!(result1.contains(&59));

    let test2 = FloatyMask::from_str("00000000000000000000000000000000X0XX").unwrap();
    assert_eq!(3, test2.floating_bits.len());
    let result2 = test2.mask(26);
    assert_eq!(8, result2.len());
    assert!(result2.contains(&16));
    assert!(result2.contains(&17));
    assert!(result2.contains(&18));
    assert!(result2.contains(&19));
    assert!(result2.contains(&24));
    assert!(result2.contains(&25));
    assert!(result2.contains(&26));
    assert!(result2.contains(&27));
}

fn main() -> Result<(), TopLevelError> {
    let filename = env::args().nth(1).expect("No file argument given.");
    let contents = fs::read_to_string(filename)?;
    let mut computer1 = Computer::<Mask>::new();
    let mut computer2 = Computer::<FloatyMask>::new();

    for line in contents.lines() {
        computer1.step(Command::<Mask>::from_str(line)?);
        computer2.step(Command::<FloatyMask>::from_str(line)?);
    }

    println!(
        "Sum of all memory values in computer #1: {}",
        computer1.locations.values().sum::<u64>()
    );
    println!(
        "Sum of all memory values in computer #2: {}",
        computer2.locations.values().sum::<u64>()
    );
    Ok(())
}
