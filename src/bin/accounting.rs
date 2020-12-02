use advent2020::errors::TopLevelError;
use std::cmp::Ordering;
use std::env;
use std::fs;
use std::str::FromStr;

fn real_main() -> Result<(), TopLevelError> {
    let mut numbers = Vec::new();

    for argument in env::args().skip(1) {
        let contents = fs::read_to_string(argument)?;
        for line in contents.lines() {
            match u64::from_str(line) {
                Err(e) => eprintln!("Skipping line with '{}': {}", line, e),
                Ok(v) => numbers.push(v),
            }
        }
    }

    // sort the arguments for faster searching
    numbers.sort_unstable();

    let mut found_right = false;
    let mut found_middle_right = false;

    'top: for i in 0..numbers.len() {
        let left = numbers[i];

        if !found_right {
            if let Some(right) = find_right(&numbers, left, &[i]) {
                println!(
                    "Found {} + {} = 2020, so {} * {} = {}",
                    left,
                    right,
                    left,
                    right,
                    left * right
                );
                found_right = true;
            }
        }

        if !found_middle_right {
            for j in 0..numbers.len() {
                if i != j {
                    let middle = numbers[j];

                    if left + middle > 2020 {
                        continue;
                    }

                    if let Some(right) = find_right(&numbers, left + middle, &[i, j]) {
                        println!(
                            "Found {} + {} + {} = 2020, so {} * {} * {} = {}",
                            left,
                            middle,
                            right,
                            left,
                            middle,
                            right,
                            left * middle * right
                        );
                        found_middle_right = true;
                        continue 'top;
                    }
                }
            }
        }
    }

    if found_right && found_middle_right {
        Ok(())
    } else {
        Err(TopLevelError::NoSolutionFound)
    }
}

fn find_right(items: &[u64], left: u64, avoid: &[usize]) -> Option<u64> {
    let mut low_tide = 0;
    let mut high_tide = items.len() - 1;

    loop {
        let target = next_target(low_tide, high_tide, avoid)?;
        let sum = left + items[target];

        match sum.cmp(&2020) {
            Ordering::Less => low_tide = target + 1,
            Ordering::Greater if target == 0 => return None,
            Ordering::Greater => high_tide = target - 1,
            Ordering::Equal => return Some(items[target]),
        }
    }
}

fn next_target(low_index: usize, high_index: usize, avoid: &[usize]) -> Option<usize> {
    if low_index > high_index {
        return None;
    }

    let midpoint = (low_index + high_index) / 2;
    let mut worker = midpoint;

    while worker >= low_index {
        if avoid.contains(&worker) {
            worker -= 1;
        } else {
            return Some(worker);
        }
    }

    worker = midpoint;
    while worker <= high_index {
        if avoid.contains(&worker) {
            worker += 1;
        } else {
            return Some(worker);
        }
    }

    None
}

fn main() {
    match real_main() {
        Err(e) => eprintln!("ERROR: {}", e),
        Ok(_) => {}
    }
}
