use advent2020::errors::TopLevelError;

fn round(x: usize, subject_number: usize) -> usize {
    (x * subject_number) % 20201227
}

fn find_rounds(subject_number: usize, target: usize) -> usize {
    let mut round_no = 1;
    let mut value = subject_number;

    while value != target {
       value = round(value, subject_number);
       round_no += 1; 
    }

    round_no
}

fn compute_key(rounds: usize, subject_number: usize) -> usize {
    let mut value = 1;

    for _ in 0..rounds {
        value = round(value, subject_number);
    }

    value
}

fn compute_encryption_key(card_public: usize, door_public: usize) -> usize {
    println!("Computing encryption key for CARD {} / DOOR {}", card_public, door_public);
    let card_rounds = find_rounds(7, card_public);
    let door_rounds = find_rounds(7, door_public);
    println!("  card rounds: {}", card_rounds);
    println!("  door rounds: {}", door_rounds);
    let card_key = compute_key(card_rounds, door_public);
    let door_key = compute_key(door_rounds, card_public);
    println!("  card key: {}", card_key);
    println!("  door key: {}", door_key);
    assert_eq!(card_key, door_key);
    card_key
}

fn main() -> Result<(), TopLevelError> {
    println!("---- TEST CASE ----");
    let test_key = compute_encryption_key(5764801, 17807724);
    println!();
    println!("---- REAL INPUT ----");
    let real_key = compute_encryption_key(12090988, 240583);
    println!();
    Ok(())
}