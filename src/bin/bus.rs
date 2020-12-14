use advent2020::errors::TopLevelError;
use std::env;
use std::fs;
use std::str::FromStr;

fn find_pattern(busses: &[(usize, usize)]) -> Result<usize, TopLevelError> {
    if busses.is_empty() {
        return Err(TopLevelError::NoInputFound);
    }

    let mut t = 1;
    let mut increment = 1;

    for (offset, factor) in busses {
        while (t + offset) % factor != 0 {
            t += increment;
        }
        increment = lcm(increment, *factor);
    }

    Ok(t)
}

fn lcm(x: usize, y: usize) -> usize {
    let mut k = x;

    while k % y != 0 {
        k += x;
    }

    k
}

#[test]
fn lcm_test() {
    assert_eq!(9, lcm(3, 9));
    assert_eq!(12, lcm(3, 4));
    assert_eq!(60, lcm(15, 20));
    assert_eq!(7, lcm(1, 7));
    assert_eq!(7, lcm(7, 1));
}

#[test]
fn pattern_examples() {
    assert_eq!(
        1068781,
        find_pattern(&[(0, 7), (1, 13), (4, 59), (6, 31), (7, 19)]).unwrap()
    );
    assert_eq!(3417, find_pattern(&[(0, 17), (2, 13), (3, 19)]).unwrap());
    assert_eq!(
        754018,
        find_pattern(&[(0, 67), (1, 7), (2, 59), (3, 61)]).unwrap()
    );
    assert_eq!(
        779210,
        find_pattern(&[(0, 67), (2, 7), (3, 59), (4, 61)]).unwrap()
    );
    assert_eq!(
        1261476,
        find_pattern(&[(0, 67), (1, 7), (3, 59), (4, 61)]).unwrap()
    );
    assert_eq!(
        1202161486,
        find_pattern(&[(0, 1789), (1, 37), (2, 47), (3, 1889)]).unwrap()
    );
}

fn main() -> Result<(), TopLevelError> {
    let filename = env::args().nth(1).expect("No file argument given.");
    let contents = fs::read_to_string(filename)?;
    let mut lines = contents.lines();
    let start_time = usize::from_str(lines.next().ok_or(TopLevelError::NoInputFound)?)?;
    let bus_notes = lines.next().ok_or(TopLevelError::NoInputFound)?;
    let mut busses = Vec::new();

    for (idx, bus) in bus_notes.split(',').enumerate() {
        match usize::from_str(bus) {
            Err(_) if bus == "x" => {}
            Err(e) => return Err(TopLevelError::NumConversionError(e)),
            Ok(x) => busses.push((idx, x)),
        }
    }

    let (next_bus, when) = busses
        .iter()
        .map(|(_, x)| (x, x - (start_time % x)))
        .min_by(|(_, a), (_, b)| a.cmp(b))
        .ok_or(TopLevelError::NoSolutionFound)?;
    println!(
        "The next #{} bus is in {} minutes. [{}]",
        next_bus,
        when,
        next_bus * when
    );
    println!(
        "The pattern you're looking for starts at {}",
        find_pattern(&busses)?
    );

    Ok(())
}
