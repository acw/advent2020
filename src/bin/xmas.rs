use advent2020::errors::TopLevelError;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(Clone)]
struct XmasChecker {
    preamble_length: usize,
    window: VecDeque<usize>,
    buffer: Vec<usize>,
}

impl XmasChecker {
    fn new(preamble_length: usize) -> XmasChecker {
        XmasChecker {
            preamble_length,
            window: VecDeque::with_capacity(preamble_length + 1),
            buffer: Vec::with_capacity(preamble_length * 10),
        }
    }

    fn push(&mut self, value: usize) -> bool {
        self.buffer.push(value);
        if self.window.len() != self.preamble_length {
            self.window.push_back(value);
            true
        } else {
            self.window.push_back(value);
            for i in 0..self.preamble_length {
                for j in 0..self.preamble_length {
                    if i != j {
                        let sum = self.window[i] + self.window[j];

                        if value == sum {
                            let _ = self.window.pop_front();
                            return true;
                        }
                    }
                }
            }
            let _ = self.window.pop_front();
            false
        }
    }

    fn find_range_summing_to(&self, value: usize) -> Option<RangeInclusive<usize>> {
        for i in 0..self.buffer.len() {
            for j in i + 1..self.buffer.len() {
                let possible_range = i..=j;
                let possible_answer: usize = self.buffer[possible_range].iter().sum();

                if possible_answer == value {
                    return Some(i..=j);
                }

                if possible_answer > value {
                    break;
                }
            }
        }

        None
    }
}

#[test]
fn first_example() {
    let contents = fs::read_to_string("inputs/day9_test1.txt");
    let mut xmas_checker = XmasChecker::new(25);

    for line in contents.unwrap().lines() {
        let next = usize::from_str(line).unwrap();
        assert!(xmas_checker.push(next));
    }

    assert!(xmas_checker.clone().push(26));
    assert!(xmas_checker.clone().push(49));
    assert!(!xmas_checker.clone().push(100));
    assert!(!xmas_checker.clone().push(50));

    // add the 45
    assert!(xmas_checker.push(45));

    assert!(xmas_checker.clone().push(26));
    assert!(!xmas_checker.clone().push(65));
    assert!(xmas_checker.clone().push(64));
    assert!(xmas_checker.clone().push(66));
}

#[test]
fn second_example() {
    let contents = fs::read_to_string("inputs/day9_test2.txt");
    let mut xmas_checker = XmasChecker::new(5);

    for line in contents.unwrap().lines() {
        let next = usize::from_str(line).unwrap();
        let result = xmas_checker.push(next);
        assert!((next == 127) || result);
    }
}

#[test]
fn third_example() {
    let contents = fs::read_to_string("inputs/day9_test2.txt");
    let mut xmas_checker = XmasChecker::new(5);

    for line in contents.unwrap().lines() {
        let next = usize::from_str(line).unwrap();
        let result = xmas_checker.push(next);
        assert!((next == 127) || result);
    }

    assert_eq!(Some(2..=5), xmas_checker.find_range_summing_to(127));
}

fn main() -> Result<(), TopLevelError> {
    let filename = env::args().skip(1).next().expect("No file argument given.");
    let contents = fs::read_to_string(filename)?;
    let mut xmas_checker = XmasChecker::new(25);

    let mut first_bad_entry = None;

    for line in contents.lines() {
        let next = usize::from_str(line).unwrap();

        if !xmas_checker.push(next) {
            println!("Entry {} fails.", next);
            if first_bad_entry.is_none() {
                first_bad_entry = Some(next);
            }
        }
    }

    if let Some(bad_entry) = first_bad_entry {
        if let Some(range) = xmas_checker.find_range_summing_to(bad_entry) {
            println!("  resulting range: {:?}", range);
            let minimum_entry = xmas_checker.buffer[range.clone()]
                .iter()
                .min()
                .ok_or(TopLevelError::UnknownError)?;
            println!("  minimum entry: {}", minimum_entry);
            let maximum_entry = xmas_checker.buffer[range]
                .iter()
                .max()
                .ok_or(TopLevelError::UnknownError)?;
            println!("  maximum entry: {}", maximum_entry);
            println!("  sum: {}", minimum_entry + maximum_entry);
            return Ok(());
        }
    }

    Err(TopLevelError::NoSolutionFound)
}
