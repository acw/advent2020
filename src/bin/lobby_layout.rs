use advent2020::errors::{DirectionParseError, MapOperationError, TopLevelError};
use std::env;
use std::fs;

#[derive(Debug)]
enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

impl Direction {
    fn read<I: Iterator<Item = char>>(
        iter: &mut I,
    ) -> Result<Option<Direction>, DirectionParseError> {
        match iter.next() {
            None => Ok(None),
            Some('e') => Ok(Some(Direction::East)),
            Some('w') => Ok(Some(Direction::West)),
            Some('n') => match iter.next() {
                None => Err(DirectionParseError::IncompleteNorthSouthDirection),
                Some('e') => Ok(Some(Direction::NorthEast)),
                Some('w') => Ok(Some(Direction::NorthWest)),
                Some(c) => Err(DirectionParseError::InvalidNorthSouthSuffix(c)),
            },
            Some('s') => match iter.next() {
                None => Err(DirectionParseError::IncompleteNorthSouthDirection),
                Some('e') => Ok(Some(Direction::SouthEast)),
                Some('w') => Ok(Some(Direction::SouthWest)),
                Some(c) => Err(DirectionParseError::InvalidNorthSouthSuffix(c)),
            },
            Some(c) => Err(DirectionParseError::InvalidBaseDirection(c)),
        }
    }

    fn adjust_coord(
        &self,
        x: &mut usize,
        y: &mut usize,
        extent: usize,
    ) -> Result<(), MapOperationError> {
        let unshifted = (*y & 1) == 0; // whether this row has been "shifted left"

        match self {
            Direction::East if *x + 1 == extent => return Err(MapOperationError::FellOffEdge),
            Direction::East => *x += 1,

            Direction::West if *x == 0 => return Err(MapOperationError::FellOffEdge),
            Direction::West => *x -= 1,

            Direction::NorthEast if unshifted && (*x + 1 == extent || *y == 0) => {
                return Err(MapOperationError::FellOffEdge)
            }
            Direction::NorthEast if unshifted => {
                *x += 1;
                *y -= 1
            }
            Direction::NorthWest if unshifted && *y == 0 => {
                return Err(MapOperationError::FellOffEdge)
            }
            Direction::NorthWest if unshifted => *y -= 1,

            Direction::SouthEast if unshifted && (*x + 1 == extent || *y + 1 == extent) => {
                return Err(MapOperationError::FellOffEdge)
            }
            Direction::SouthEast if unshifted => {
                *x += 1;
                *y += 1;
            }
            Direction::SouthWest if unshifted && *y + 1 == extent => {
                return Err(MapOperationError::FellOffEdge)
            }
            Direction::SouthWest if unshifted => *y += 1,

            Direction::NorthEast if *y == 0 => return Err(MapOperationError::FellOffEdge),
            Direction::NorthEast => *y -= 1,
            Direction::NorthWest if *x == 0 || *y == 0 => {
                return Err(MapOperationError::FellOffEdge)
            }
            Direction::NorthWest => {
                *x -= 1;
                *y -= 1;
            }

            Direction::SouthEast if *y + 1 == extent => return Err(MapOperationError::FellOffEdge),
            Direction::SouthEast => *y += 1,
            Direction::SouthWest if *x == 0 || *y + 1 == extent => {
                return Err(MapOperationError::FellOffEdge)
            }
            Direction::SouthWest => {
                *x -= 1;
                *y += 1
            }
        }
        Ok(())
    }
}

fn read_directions<I: Iterator<Item = char>>(
    iter: &mut I,
) -> Result<Vec<Direction>, DirectionParseError> {
    let mut directions = Vec::new();

    while let Some(x) = Direction::read(iter)? {
        directions.push(x);
    }

    Ok(directions)
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Color {
    White,
    Black,
}

impl Color {
    fn flip(&mut self) {
        match self {
            Color::White => *self = Color::Black,
            Color::Black => *self = Color::White,
        }
    }
}

#[derive(Clone)]
struct Board {
    edge_length: usize,
    origin: (usize, usize),
    raw_data: Vec<Color>,
}

macro_rules! check_adjacent {
    ($self: expr, $x: expr, $y: expr, $dir: expr, $val: ident) => {
        let mut work_x = $x;
        let mut work_y = $y;

        if $dir
            .adjust_coord(&mut work_x, &mut work_y, $self.edge_length)
            .is_ok()
            && $self.get(work_x, work_y) == Color::Black
        {
            $val += 1;
        }
    };
}

impl Board {
    fn new(edge_length: usize) -> Board {
        let edge_length_squared = edge_length * edge_length;
        let half_edge_length = edge_length / 2;
        let origin = (half_edge_length, half_edge_length);
        let raw_data = vec![Color::White; edge_length_squared];

        Board {
            edge_length,
            origin,
            raw_data,
        }
    }

    fn tile_counts(&self) -> (usize, usize) {
        let mut black_count = 0;
        let mut white_count = 0;

        for color in self.raw_data.iter() {
            match color {
                Color::White => white_count += 1,
                Color::Black => black_count += 1,
            }
        }

        (black_count, white_count)
    }

    fn flip(&mut self, directions: Vec<Direction>) -> Result<(), TopLevelError> {
        let (mut x, mut y) = self.origin;

        for direction in directions.iter() {
            direction.adjust_coord(&mut x, &mut y, self.edge_length)?;
        }

        self.raw_data[(y * self.edge_length) + x].flip();

        Ok(())
    }

    fn get(&self, x: usize, y: usize) -> Color {
        self.raw_data[(y * self.edge_length) + x]
    }

    fn set(&mut self, x: usize, y: usize, v: Color) {
        self.raw_data[(y * self.edge_length) + x] = v;
    }

    fn adjacent_black_tiles(&self, x: usize, y: usize) -> usize {
        let mut result = 0;

        check_adjacent!(self, x, y, Direction::East, result);
        check_adjacent!(self, x, y, Direction::West, result);
        check_adjacent!(self, x, y, Direction::NorthEast, result);
        check_adjacent!(self, x, y, Direction::NorthWest, result);
        check_adjacent!(self, x, y, Direction::SouthEast, result);
        check_adjacent!(self, x, y, Direction::SouthWest, result);

        result
    }

    fn next_day(self) -> Board {
        let mut result = self.clone();

        for x in 0..self.edge_length {
            for y in 0..self.edge_length {
                let black_count = self.adjacent_black_tiles(x, y);

                match self.get(x, y) {
                    Color::Black if black_count == 0 || black_count > 2 => {
                        result.set(x, y, Color::White)
                    }
                    Color::White if black_count == 2 => result.set(x, y, Color::Black),
                    _ => {}
                }
            }
        }

        result
    }
}

#[test]
fn maneuvers_work() {
    let mut board = Board::new(10);

    board.flip(vec![Direction::West]).unwrap();
    assert_eq!(1, board.tile_counts().0);
    board.flip(vec![Direction::West]).unwrap();
    assert_eq!(0, board.tile_counts().0);
    board
        .flip(vec![Direction::West, Direction::West, Direction::East])
        .unwrap();
    assert_eq!(1, board.tile_counts().0);
    board.flip(vec![Direction::West]).unwrap();
    assert_eq!(0, board.tile_counts().0);
    board
        .flip(read_directions(&mut "esew".chars()).unwrap())
        .unwrap();
    assert_eq!(1, board.tile_counts().0);
    board.flip(vec![Direction::SouthEast]).unwrap();
    assert_eq!(0, board.tile_counts().0);
    board
        .flip(read_directions(&mut "nwwswee".chars()).unwrap())
        .unwrap();
    assert_eq!(1, board.tile_counts().0);
    board.flip(vec![]).unwrap();
    assert_eq!(0, board.tile_counts().0);
    board
        .flip(read_directions(&mut "nwnwwswee".chars()).unwrap())
        .unwrap();
    assert_eq!(1, board.tile_counts().0);
    board.flip(vec![Direction::NorthWest]).unwrap();
    assert_eq!(0, board.tile_counts().0);
}

fn main() -> Result<(), TopLevelError> {
    let filename = env::args().nth(1).expect("No file argument given.");
    let contents = fs::read_to_string(filename)?;
    let mut board = Board::new(250);

    for line in contents.lines() {
        board.flip(read_directions(&mut line.chars())?)?;
    }

    let (black_count, white_count) = board.tile_counts();
    println!("{} black tiles.", black_count);
    println!("{} white tiles.", white_count);

    for i in 0..=100 {
        println!("Day {}: {} black tiles", i, board.tile_counts().0);
        board = board.next_day();
    }

    Ok(())
}
