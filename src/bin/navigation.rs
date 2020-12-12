use advent2020::errors::{IllegalFerryCommand, TopLevelError};
use std::env;
use std::fs;
use std::str::FromStr;

struct Ferry {
    x: isize,
    y: isize,
    direction: Direction,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn left(&self, mut amt: usize) -> Direction {
        let mut res = *self;

        assert_eq!(amt % 90, 0);
        while amt > 0 {
            res = match res {
                Direction::North => Direction::West,
                Direction::East => Direction::North,
                Direction::South => Direction::East,
                Direction::West => Direction::South,
            };
            amt -= 90;
        }

        res
    }

    fn right(&self, mut amt: usize) -> Direction {
        let mut res = *self;

        assert_eq!(amt % 90, 0);
        while amt > 0 {
            res = match res {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
            };
            amt -= 90;
        }

        res
    }
}

enum Command {
    TurnLeft(usize),
    TurnRight(usize),
    GoForward(usize),
    Shift(Direction, usize),
}

impl FromStr for Command {
    type Err = IllegalFerryCommand;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(IllegalFerryCommand::EmptyCommand);
        }

        match s.split_at(1) {
            ("N", n) => Ok(Command::Shift(Direction::North, usize::from_str(n)?)),
            ("E", n) => Ok(Command::Shift(Direction::East, usize::from_str(n)?)),
            ("S", n) => Ok(Command::Shift(Direction::South, usize::from_str(n)?)),
            ("W", n) => Ok(Command::Shift(Direction::West, usize::from_str(n)?)),
            ("L", n) => Ok(Command::TurnLeft(usize::from_str(n)?)),
            ("R", n) => Ok(Command::TurnRight(usize::from_str(n)?)),
            ("F", n) => Ok(Command::GoForward(usize::from_str(n)?)),
            (x, _) => Err(IllegalFerryCommand::UnknownCommand(
                x.chars().next().ok_or(IllegalFerryCommand::EmptyCommand)?,
            )),
        }
    }
}

impl Ferry {
    fn new() -> Ferry {
        Ferry {
            x: 0,
            y: 0,
            direction: Direction::East,
        }
    }

    fn go(&mut self, cmd: &Command) {
        match cmd {
            Command::Shift(Direction::North, v) => self.y += *v as isize,
            Command::Shift(Direction::East, v) => self.x += *v as isize,
            Command::Shift(Direction::South, v) => self.y -= *v as isize,
            Command::Shift(Direction::West, v) => self.x -= *v as isize,
            Command::TurnLeft(amt) => self.direction = self.direction.left(*amt),
            Command::TurnRight(amt) => self.direction = self.direction.right(*amt),
            Command::GoForward(amt) => self.go(&Command::Shift(self.direction, *amt)),
        }
    }

    fn travel_manhattan_distance(&self) -> usize {
        (self.y.abs() + self.x.abs()) as usize
    }
}

struct GuidedFerry {
    rise: isize,
    run: isize,
    x: isize,
    y: isize,
}

fn stupid_linear_algebra(mut theta: usize, x: &mut isize, y: &mut isize) {
    while theta > 0 {
        let inx = *x;
        *x = -*y;
        *y = inx;
        theta -= 90;
    }
}

impl GuidedFerry {
    fn new() -> GuidedFerry {
        GuidedFerry {
            rise: 1,
            run: 10,
            x: 0,
            y: 0,
        }
    }

    fn go(&mut self, cmd: &Command) {
        match cmd {
            Command::Shift(Direction::North, v) => self.rise += *v as isize,
            Command::Shift(Direction::East, v) => self.run += *v as isize,
            Command::Shift(Direction::South, v) => self.rise -= *v as isize,
            Command::Shift(Direction::West, v) => self.run -= *v as isize,
            Command::TurnLeft(amt) => stupid_linear_algebra(*amt, &mut self.run, &mut self.rise),
            Command::TurnRight(amt) => {
                stupid_linear_algebra(360 - amt, &mut self.run, &mut self.rise)
            }
            Command::GoForward(amt) => {
                self.x += self.run * (*amt as isize);
                self.y += self.rise * (*amt as isize);
            }
        }
    }

    fn travel_manhattan_distance(&self) -> usize {
        (self.y.abs() + self.x.abs()) as usize
    }
}

fn main() -> Result<(), TopLevelError> {
    let filename = env::args().nth(1).expect("No file argument given.");
    let contents = fs::read_to_string(filename)?;
    let mut ferry = Ferry::new();
    let mut guided_ferry = GuidedFerry::new();

    for line in contents.lines() {
        let command = Command::from_str(line)?;
        ferry.go(&command);
        guided_ferry.go(&command);
    }

    println!(
        "Base ferry at ({}, {}), travelled Manhattan distance: {}",
        ferry.x,
        ferry.y,
        ferry.travel_manhattan_distance()
    );
    println!(
        "Guided ferry at ({}, {}), travelled Manhattan distance: {}",
        guided_ferry.x,
        guided_ferry.y,
        guided_ferry.travel_manhattan_distance()
    );

    Ok(())
}
