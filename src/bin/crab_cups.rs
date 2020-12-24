use advent2020::errors::TopLevelError;
use std::fmt;
use std::time::SystemTime;

#[derive(Clone)]
struct CupSlot {
    value: usize,
    next: usize,
}

impl CupSlot {
    fn blank() -> CupSlot {
        CupSlot { value: 0, next: 0 }
    }
}

struct CupGame {
    // this is a slightly wacky encoding; element 0 tells you which number
    // comes first, the rest of the elements tell you which number comes
    // after the indexed one. So mappings[1] tells you which number comes
    // after the element 1 in the list.
    nodes: Vec<CupSlot>,
    value_indices: Vec<usize>,
    round: usize,
    current_cup: usize,
    maximum_value: usize,
}

impl CupGame {
    fn new(problem_size: usize, initial_cups: &[usize]) -> CupGame {
        let mut nodes = vec![CupSlot::blank(); problem_size + 1];
        let mut value_indices = vec![0; problem_size + 1];
        let mut previous = 0;
        let mut current = 1;
        let mut maximum_value = 0;

        for value in initial_cups.iter() {
            nodes[current].value = *value;
            value_indices[*value] = current;
            nodes[previous].next = current;
            if value > &maximum_value {
                maximum_value = *value;
            }
            previous = current;
            current += 1;
        }

        while current <= problem_size {
            maximum_value += 1;
            value_indices[maximum_value] = current;
            nodes[previous].next = current;
            nodes[current].value = maximum_value;
            previous = current;
            current += 1;
        }

        nodes[previous].next = 1;

        CupGame {
            nodes,
            value_indices,
            maximum_value,
            round: 1,
            current_cup: 1,
        }
    }

    fn pull_next(&mut self) -> (usize, usize, usize) {
        let a_idx = self.nodes[self.current_cup].next;
        let b_idx = self.nodes[a_idx].next;
        let c_idx = self.nodes[b_idx].next;

        self.nodes[self.current_cup].next = self.nodes[c_idx].next;

        (a_idx, b_idx, c_idx)
    }

    fn destination_cup(&self, pulled: &(usize, usize, usize)) -> usize {
        let mut proposed_value = self.nodes[self.current_cup].value;
        let (a_idx, b_idx, c_idx) = pulled;
        let a = self.nodes[*a_idx].value;
        let b = self.nodes[*b_idx].value;
        let c = self.nodes[*c_idx].value;

        loop {
            if proposed_value == 1 {
                proposed_value = self.maximum_value;
            } else {
                proposed_value -= 1;
            }

            if (proposed_value != a) && (proposed_value != b) && (proposed_value != c) {
                return proposed_value;
            }
        }
    }

    fn reinject(&mut self, at_value: usize, pulled: (usize, usize, usize)) {
        let (a_idx, b_idx, c_idx) = pulled;
        let idx = self.value_indices[at_value];

        self.nodes[a_idx].next = b_idx;
        self.nodes[b_idx].next = c_idx;
        self.nodes[c_idx].next = self.nodes[idx].next;
        self.nodes[idx].next = a_idx;
    }

    fn run_round(&mut self) {
        let pull = self.pull_next();
        let destination_cup = self.destination_cup(&pull);
        self.reinject(destination_cup, pull);
        self.current_cup = self.nodes[self.current_cup].next;
        self.round += 1;
    }

    fn part1_answer(&self) -> String {
        let idx = self.value_indices[1];
        let mut retval = String::new();
        let mut work = self.nodes[idx].next;

        while self.nodes[work].value != 1 {
            retval.push_str(format!("{}", self.nodes[work].value).as_str());
            work = self.nodes[work].next;
        }

        retval
    }

    fn part2_answer(&self) -> usize {
        let idx = self.value_indices[1];
        let a_idx = self.nodes[idx].next;
        let b_idx = self.nodes[a_idx].next;
        let a = self.nodes[a_idx].value;
        let b = self.nodes[b_idx].value;
        println!("{} * {} = {}", a, b, a * b);
        a * b
    }
}

impl fmt::Display for CupGame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Round {}: ", self.round)?;
        let mut idx = self.current_cup;
        let first_value = self.nodes[idx].value;

        write!(f, "({}) ", first_value)?;
        idx = self.nodes[idx].next;
        while self.nodes[idx].value != first_value {
            write!(f, "{} ", self.nodes[idx].value)?;
            idx = self.nodes[idx].next;
        }

        Ok(())
    }
}

fn main() -> Result<(), TopLevelError> {
    let initial_cups = &[3, 6, 8, 1, 9, 5, 7, 4, 2];
    let mut game = CupGame::new(9, initial_cups);

    for _ in 0..100 {
        println!("{}", game);
        game.run_round();
    }
    println!("Part 1 answer: {:?}", game.part1_answer());

    let start_time = SystemTime::now();
    let mut last_chunk = start_time;
    let mut game2 = CupGame::new(1_000_000, initial_cups);
    for i in 0..10_000_000 - 100 {
        if i % 1_000_000 == 0 {
            let now = SystemTime::now();
            let since_start = now.duration_since(start_time).unwrap().as_secs();
            let since_last = now.duration_since(last_chunk).unwrap().as_secs();

            println!(
                "Round {}: {}m{}s since start, {}m{}s for this chunk",
                i,
                since_start / 60,
                since_start % 60,
                since_last / 60,
                since_last % 60
            );
            last_chunk = now;
        }
        game2.run_round();
    }
    println!("Part 2 answer: {}", game2.part2_answer());

    Ok(())
}
