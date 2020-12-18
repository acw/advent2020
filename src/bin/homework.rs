use advent2020::errors::TopLevelError;
use advent2020::math::Math;
use std::env;
use std::fs;

#[test]
fn neutral_examples() {
    assert_eq!(
        71,
        Math::new_neutral("1 + 2 * 3 + 4 * 5 + 6")
            .unwrap()
            .compute()
    );
    assert_eq!(
        51,
        Math::new_neutral("1 + (2 * 3) + (4 * (5 + 6))")
            .unwrap()
            .compute()
    );
    assert_eq!(26, Math::new_neutral("2 * 3 + (4 * 5)").unwrap().compute());
    assert_eq!(
        437,
        Math::new_neutral("5 + (8 * 3 + 9 + 3 * 4 * 3)")
            .unwrap()
            .compute()
    );
    assert_eq!(
        12240,
        Math::new_neutral("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")
            .unwrap()
            .compute()
    );
    assert_eq!(
        13632,
        Math::new_neutral("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")
            .unwrap()
            .compute()
    );
}

#[test]
fn add_first_examples() {
    assert_eq!(
        231,
        Math::new_add_first("1 + 2 * 3 + 4 * 5 + 6")
            .unwrap()
            .compute()
    );
    assert_eq!(
        51,
        Math::new_add_first("1 + (2 * 3) + (4 * (5 + 6))")
            .unwrap()
            .compute()
    );
    assert_eq!(
        46,
        Math::new_add_first("2 * 3 + (4 * 5)").unwrap().compute()
    );
    assert_eq!(
        1445,
        Math::new_add_first("5 + (8 * 3 + 9 + 3 * 4 * 3)")
            .unwrap()
            .compute()
    );
    assert_eq!(
        669060,
        Math::new_add_first("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")
            .unwrap()
            .compute()
    );
    assert_eq!(
        23340,
        Math::new_add_first("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")
            .unwrap()
            .compute()
    );
}

fn main() -> Result<(), TopLevelError> {
    let filename = env::args().nth(1).expect("No file argument given.");
    let contents = fs::read_to_string(filename)?;
    let mut sum_neutral = 0;
    let mut sum_add_first = 0;

    for line in contents.lines() {
        match Math::new_neutral(line) {
            Ok(expr) => {
                println!("{} ==> {}", line, expr.compute());
                sum_neutral += expr.compute();
            }
            Err(e) => println!("PARSE ERROR: {}", e),
        }
        match Math::new_add_first(line) {
            Ok(expr) => {
                println!("{} ==> {}", line, expr.compute());
                sum_add_first += expr.compute();
            }
            Err(e) => println!("PARSE ERROR: {}", e),
        }
    }

    println!("Total (neutral ordering): {}", sum_neutral);
    println!("Total (add-first ordering): {}", sum_add_first);

    Ok(())
}
