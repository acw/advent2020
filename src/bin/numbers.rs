use advent2020::errors::TopLevelError;
use std::collections::HashMap;
struct Game {
    history: HashMap<usize, Vec<usize>>,
    on_turn: usize,
    last_value: usize,
}

impl Game {
    fn new(starting_values: &[usize]) -> Result<Game, TopLevelError> {
        if starting_values.is_empty() {
            return Err(TopLevelError::NoInputFound);
        }

        let mut history = HashMap::new();
        let mut on_turn = 0;
        let mut last_value = 0;

        for value in starting_values {
            match history.get_mut(value) {
                None => {
                    let _ = history.insert(*value, vec![on_turn]);
                }
                Some(v) => v.push(on_turn),
            }
            on_turn += 1;
            last_value = *value;
        }

        Ok(Game {
            history,
            on_turn,
            last_value,
        })
    }

    fn add_to_history(&mut self, value: usize) {
        match self.history.get_mut(&value) {
            None => {
                let _ = self.history.insert(value, vec![self.on_turn]);
            }
            Some(v) if v.len() == 1 => v.push(self.on_turn),
            Some(v) => {
                v[0] = v[1];
                v[1] = self.on_turn;
            }
        }
    }

    fn step(&mut self) {
        let insert_history = self
            .history
            .get(&self.last_value)
            .expect("The world broke :(");

        if insert_history.len() < 2 {
            self.last_value = 0;
        } else {
            self.last_value = insert_history[1] - insert_history[0];
        }

        self.add_to_history(self.last_value);
        self.on_turn += 1;
    }

    fn run_through_turn(&mut self, final_turn: usize) -> usize {
        while self.on_turn < final_turn {
            self.step();
        }

        self.last_value
    }
}

#[test]
fn game_tests() {
    let mut test1 = Game::new(&[0, 3, 6]).unwrap();
    assert_eq!(0, test1.run_through_turn(10));
    let mut test2 = Game::new(&[1, 3, 2]).unwrap();
    assert_eq!(1, test2.run_through_turn(2020));
    let mut test3 = Game::new(&[2, 1, 3]).unwrap();
    assert_eq!(10, test3.run_through_turn(2020));
    let mut test4 = Game::new(&[1, 2, 3]).unwrap();
    assert_eq!(27, test4.run_through_turn(2020));
    let mut test5 = Game::new(&[2, 3, 1]).unwrap();
    assert_eq!(78, test5.run_through_turn(2020));
    let mut test6 = Game::new(&[3, 2, 1]).unwrap();
    assert_eq!(438, test6.run_through_turn(2020));
    let mut test7 = Game::new(&[3, 1, 2]).unwrap();
    assert_eq!(1836, test7.run_through_turn(2020));
}

fn main() -> Result<(), TopLevelError> {
    let mut game = Game::new(&[16, 1, 0, 18, 12, 14, 19])?;
    let result1 = game.run_through_turn(2020);
    println!("The 2020th number spoken is {}", result1);
    let result2 = game.run_through_turn(30000000);
    println!("The 2020th number spoken is {}", result2);
    Ok(())
}
