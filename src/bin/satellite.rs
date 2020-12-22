use advent2020::errors::{TileParseError, TopLevelError};
#[cfg(test)]
use std::collections::HashMap;
use std::env;
use std::fmt;
use std::fs;
use std::str::FromStr;

#[derive(Clone)]
struct Tile {
    identity: usize,
    history: Vec<Modification>,
    top: u16,
    bottom: u16,
    left: u16,
    right: u16,
    edge_length: usize,
    raw_data: Vec<bool>,
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.identity == other.identity && self.raw_data == other.raw_data
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.identity)?;
        let mut prefix = ':';
        for mvmt in self.history.iter() {
            write!(f, "{}{:?}", prefix, mvmt)?;
            prefix = '+';
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Modification {
    FlippedX,
    FlippedY,
    Rotated,
}

impl Tile {
    fn new(
        identity: usize,
        history: Vec<Modification>,
        edge_length: usize,
        raw_data: Vec<bool>,
    ) -> Tile {
        let mut res = Tile {
            identity,
            history,
            top: 0,
            bottom: 0,
            left: 0,
            right: 0,
            edge_length,
            raw_data,
        };

        for i in 0..edge_length {
            res.top = (res.top << 1) | res.get_value(i, 0);
            res.bottom = (res.bottom << 1) | res.get_value(i, edge_length - 1);
            res.left = (res.left << 1) | res.get_value(0, i);
            res.right = (res.right << 1) | res.get_value(edge_length - 1, i);
        }

        res
    }

    fn read<'a, I: Iterator<Item = &'a str>>(
        lines: &mut I,
    ) -> Result<Option<Tile>, TileParseError> {
        match lines.next() {
            None => Ok(None),
            Some("") => Tile::read(lines),
            Some(x) if x.starts_with("Tile ") => {
                let identity = usize::from_str(&x[5..x.len() - 1])?;
                let mut edge_length = 0;
                let mut raw_data = Vec::new();

                for line in lines {
                    if line == "" {
                        break;
                    }

                    for char in line.chars() {
                        match char {
                            '.' => raw_data.push(false),
                            '#' => raw_data.push(true),
                            _ => return Err(TileParseError::IllegalCharacter(char)),
                        }
                    }

                    edge_length += 1;
                }

                if raw_data.len() != (edge_length * edge_length) {
                    return Err(TileParseError::IllegalDimensions(identity));
                }

                Ok(Some(Tile::new(identity, vec![], edge_length, raw_data)))
            }
            Some(other) => Err(TileParseError::BadTileStart(other.to_string())),
        }
    }

    fn get(&self, x: usize, y: usize) -> bool {
        self.raw_data[(y * self.edge_length) + x]
    }

    fn get_value(&self, x: usize, y: usize) -> u16 {
        if self.get(x, y) {
            1
        } else {
            0
        }
    }

    fn set(&mut self, x: usize, y: usize, v: bool) {
        self.raw_data[(y * self.edge_length) + x] = v;
    }

    fn draw(&self) {
        println!("Tile {} [{:?}]:", self.identity, self.history);
        for y in 0..self.edge_length {
            for x in 0..self.edge_length {
                if self.get(x, y) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn flip_over_x(&self) -> Tile {
        let mut raw_data = Vec::with_capacity(self.edge_length * self.edge_length);
        let reverser = self.edge_length - 1;

        for y in 0..self.edge_length {
            for x in 0..self.edge_length {
                raw_data.push(self.get(x, reverser - y))
            }
        }

        let mut new_history = self.history.clone();
        new_history.push(Modification::FlippedX);
        Tile::new(self.identity, new_history, self.edge_length, raw_data)
    }

    fn flip_over_y(&self) -> Tile {
        let mut raw_data = Vec::with_capacity(self.edge_length * self.edge_length);
        let reverser = self.edge_length - 1;

        for y in 0..self.edge_length {
            for x in 0..self.edge_length {
                raw_data.push(self.get(reverser - x, y))
            }
        }

        let mut new_history = self.history.clone();
        new_history.push(Modification::FlippedY);
        Tile::new(self.identity, new_history, self.edge_length, raw_data)
    }

    fn rotate(&self) -> Tile {
        let mut res = self.clone();

        res.history.push(Modification::Rotated);
        for x in 0..self.edge_length {
            for y in 0..self.edge_length {
                res.set(self.edge_length - 1 - y, x, self.get(x, y));
            }
        }

        Tile::new(res.identity, res.history, res.edge_length, res.raw_data)
    }

    fn variants(self) -> Vec<Tile> {
        let mut res = vec![];
        let mut new_elements = vec![self];

        while !new_elements.is_empty() {
            res.append(&mut new_elements);

            for current in res.iter() {
                let flip_x = current.flip_over_x();
                if !res.contains(&flip_x) && !new_elements.contains(&flip_x) {
                    new_elements.push(flip_x);
                }

                let flip_y = current.flip_over_y();
                if !res.contains(&flip_y) && !new_elements.contains(&flip_y) {
                    new_elements.push(flip_y);
                }

                let rotated = current.rotate();
                if !res.contains(&rotated) && !new_elements.contains(&rotated) {
                    new_elements.push(rotated);
                }
            }
        }

        res
    }

    fn can_be_left_of(&self, other: &Tile) -> bool {
        self.identity != other.identity && self.right == other.left
    }

    fn can_be_right_of(&self, other: &Tile) -> bool {
        self.identity != other.identity && self.left == other.right
    }

    fn can_be_above(&self, other: &Tile) -> bool {
        self.identity != other.identity && self.bottom == other.top
    }

    fn can_be_below(&self, other: &Tile) -> bool {
        self.identity != other.identity && self.top == other.bottom
    }
}

#[test]
fn flip_x_test() {
    let raw_data = vec![
        true, false, true, false, true, true, true, true, false, false, true, true, false, false,
        false, false,
    ];
    let edge_length = 4;
    let identity = 1;
    let original = Tile::new(identity, vec![], edge_length, raw_data);
    let flipped = vec![
        false, false, false, false, false, false, true, true, true, true, true, true, true, false,
        true, false,
    ];

    assert_eq!(flipped, original.flip_over_x().raw_data);
}

#[test]
fn flip_y_test() {
    let raw_data = vec![
        true, false, true, false, true, true, true, true, false, false, true, true, false, false,
        false, false,
    ];
    let edge_length = 4;
    let identity = 1;
    let original = Tile::new(identity, vec![], edge_length, raw_data);
    let flipped = vec![
        false, true, false, true, true, true, true, true, true, true, false, false, false, false,
        false, false,
    ];

    assert_eq!(flipped, original.flip_over_y().raw_data);
}

#[test]
fn rotate_test() {
    let original = Tile::new(
        0,
        vec![],
        3,
        vec![true, true, true, false, false, false, true, true, true],
    );
    let rotated = vec![true, false, true, true, false, true, true, false, true];
    assert_eq!(rotated, original.rotate().raw_data);
}

#[test]
fn next_to_tests() {
    let contents = fs::read_to_string("inputs/day20_test.txt").unwrap();
    let mut lines = contents.lines();
    let mut tiles = HashMap::new();

    while let Some(new_tile) = Tile::read(&mut lines).unwrap() {
        tiles.insert(new_tile.identity, new_tile);
    }

    let tile1951 = tiles.get(&1951).unwrap().flip_over_x();
    let tile2729 = tiles.get(&2729).unwrap().flip_over_x();
    let tile2971 = tiles.get(&2971).unwrap().flip_over_x();
    let tile2311 = tiles.get(&2311).unwrap().flip_over_x();
    let tile1427 = tiles.get(&1427).unwrap().flip_over_x();
    let tile1489 = tiles.get(&1489).unwrap().flip_over_x();
    let tile3079 = tiles.get(&3079).unwrap().clone();
    let tile2473 = tiles.get(&2473).unwrap().flip_over_y().rotate();
    let tile1171 = tiles.get(&1171).unwrap().flip_over_y();

    tile1951.draw();
    println!();
    tile2729.draw();
    println!();
    tile2971.draw();
    println!();
    tile2311.draw();
    println!();
    tile1427.draw();
    println!();
    tile1489.draw();
    println!();
    tile3079.draw();
    println!();
    tile2473.draw();
    println!();
    tile1171.draw();
    println!();

    // above tests
    assert!(tile1951.can_be_above(&tile2729));
    assert!(tile2729.can_be_above(&tile2971));
    assert!(tile2311.can_be_above(&tile1427));
    assert!(tile1427.can_be_above(&tile1489));
    assert!(tile3079.can_be_above(&tile2473));
    assert!(tile2473.can_be_above(&tile1171));

    // below tests
    assert!(tile2729.can_be_below(&tile1951));
    assert!(tile2971.can_be_below(&tile2729));
    assert!(tile1427.can_be_below(&tile2311));
    assert!(tile1489.can_be_below(&tile1427));
    assert!(tile2473.can_be_below(&tile3079));
    assert!(tile1171.can_be_below(&tile2473));

    // left tests
    assert!(tile1951.can_be_left_of(&tile2311));
    assert!(tile2729.can_be_left_of(&tile1427));
    assert!(tile2971.can_be_left_of(&tile1489));
    assert!(tile2311.can_be_left_of(&tile3079));
    assert!(tile1427.can_be_left_of(&tile2473));
    assert!(tile1489.can_be_left_of(&tile1171));

    // right tests
    assert!(tile2311.can_be_right_of(&tile1951));
    assert!(tile1427.can_be_right_of(&tile2729));
    assert!(tile1489.can_be_right_of(&tile2971));
    assert!(tile3079.can_be_right_of(&tile2311));
    assert!(tile2473.can_be_right_of(&tile1427));
    assert!(tile1171.can_be_right_of(&tile1489));
}

#[derive(Clone)]
struct Board {
    edge_length: usize,
    raw_data: Vec<Vec<Tile>>,
}

impl Board {
    fn new(original_tile_count: usize, all_variants: Vec<Tile>) -> Board {
        let mut edge_length = 1;

        while edge_length * edge_length < original_tile_count {
            edge_length += 1;
        }

        let mut raw_data = Vec::with_capacity(original_tile_count);
        raw_data.resize(original_tile_count, all_variants);

        Board {
            edge_length,
            raw_data,
        }
    }

    fn get(&self, x: usize, y: usize) -> &[Tile] {
        &self.raw_data[(y * self.edge_length) + x]
    }

    fn set(&mut self, x: usize, y: usize, v: Vec<Tile>) {
        self.raw_data[(y * self.edge_length) + x] = v;
    }

    fn print_status(&self) {
        for y in 0..self.edge_length {
            for x in 0..self.edge_length {
                print!("{}\t", self.get(x, y).len());
            }
            println!();
        }
    }

    fn reduce(&mut self) -> bool {
        let mut removed_something = false;

        for x in 0..self.edge_length {
            for y in 0..self.edge_length {
                let mut new_possibles = Vec::new();

                for possible in self.get(x, y) {
                    let mut all_ok = true;

                    if x > 0 {
                        all_ok &= self
                            .get(x - 1, y)
                            .iter()
                            .any(|other| possible.can_be_right_of(other));
                    }

                    if y > 0 {
                        all_ok &= self
                            .get(x, y - 1)
                            .iter()
                            .any(|other| possible.can_be_below(other));
                    }

                    if x + 1 != self.edge_length {
                        all_ok &= self
                            .get(x + 1, y)
                            .iter()
                            .any(|other| possible.can_be_left_of(other));
                    }

                    if y + 1 != self.edge_length {
                        all_ok &= self
                            .get(x, y + 1)
                            .iter()
                            .any(|other| possible.can_be_above(other));
                    }

                    if all_ok {
                        new_possibles.push(possible.clone());
                    } else {
                        removed_something = true;
                    }
                }

                self.set(x, y, new_possibles);
            }
        }

        removed_something
    }

    fn solve(&mut self) -> Result<Board, TopLevelError> {
        // first, let's find the spot in the board with the fewest possibilities
        let mut split_x = 0;
        let mut split_y = 0;
        let mut possibilities = 0xfffffffffffffff;

        for x in 0..self.edge_length {
            for y in 0..self.edge_length {
                if self.get(x, y).len() < possibilities {
                    split_x = x;
                    split_y = y;
                    possibilities = self.get(x, y).len();
                }
            }
        }

        for split_value in self.get(split_x, split_y).iter() {
            let mut possible_board = self.clone();
            possible_board.set(split_x, split_y, vec![split_value.clone()]);
            while possible_board.reduce() {}
            if possible_board.raw_data.iter().all(|x| x.len() == 1) {
                let mut idents: Vec<usize> = possible_board
                    .raw_data
                    .iter()
                    .map(|x| x[0].identity)
                    .collect();
                let orig_length = idents.len();
                idents.sort();
                idents.dedup();
                if idents.len() == orig_length {
                    return Ok(possible_board);
                }
            }
        }

        Err(TopLevelError::NoSolutionFound)
    }
}

#[derive(Clone, PartialEq)]
struct Image {
    width: usize,
    height: usize,
    raw_data: Vec<Pixel>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Pixel {
    Empty,
    Block,
    Monster,
}

impl fmt::Display for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Pixel::Empty => write!(f, "."),
            Pixel::Block => write!(f, "#"),
            Pixel::Monster => write!(f, "O"),
        }
    }
}

impl From<Board> for Image {
    fn from(b: Board) -> Image {
        let board_edge_length = b.edge_length;
        let tile_edge_length = b.raw_data[0][0].edge_length;
        let chunk_edge_length = tile_edge_length - 2;
        let edge_length = board_edge_length * chunk_edge_length;
        let mut raw_data = Vec::with_capacity(edge_length * edge_length);
        let width = edge_length;
        let height = edge_length;

        raw_data.resize(edge_length * edge_length, Pixel::Empty);
        let mut result = Image {
            width,
            height,
            raw_data,
        };

        for board_y in 0..board_edge_length {
            for board_x in 0..board_edge_length {
                let board = &b.get(board_x, board_y)[0];

                for inner_y in 0..chunk_edge_length {
                    for inner_x in 0..chunk_edge_length {
                        let pixel_value = if board.get(inner_x + 1, inner_y + 1) {
                            Pixel::Block
                        } else {
                            Pixel::Empty
                        };

                        result.set(
                            (board_x * chunk_edge_length) + inner_x,
                            (board_y * chunk_edge_length) + inner_y,
                            pixel_value,
                        );
                    }
                }
            }
        }

        result
    }
}

impl Image {
    fn sea_monster() -> Image {
        let data = "                  # #    ##    ##    ### #  #  #  #  #  #   ";
        let raw_data = data
            .chars()
            .map(|x| match x {
                ' ' => Pixel::Empty,
                '#' => Pixel::Monster,
                _ => panic!("the world broke"),
            })
            .collect();
        Image {
            width: 20,
            height: 3,
            raw_data,
        }
    }

    fn get(&self, x: usize, y: usize) -> Pixel {
        self.raw_data[(y * self.width) + x]
    }

    fn set(&mut self, x: usize, y: usize, v: Pixel) {
        self.raw_data[(y * self.width) + x] = v;
    }

    fn draw(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.get(x, y));
            }
            println!();
        }
    }

    fn overwrite(&mut self, x: usize, y: usize, image: &Image) {
        for ix in 0..image.width {
            for iy in 0..image.height {
                if image.get(ix, iy) == Pixel::Monster {
                    self.set(x + ix, y + iy, Pixel::Monster);
                }
            }
        }
    }

    fn overlay(&mut self, image: &Image) -> usize {
        let mut monsters_found = 0;

        for x in 0..=(self.width - image.width) {
            for y in 0..=(self.height - image.height) {
                let mut all_match = true;

                for ix in 0..image.width {
                    for iy in 0..image.height {
                        all_match &= image.get(ix, iy) != Pixel::Monster
                            || self.get(x + ix, y + iy) != Pixel::Empty;
                    }
                }

                if all_match {
                    self.overwrite(x, y, &image);
                    monsters_found += 1;
                }
            }
        }

        monsters_found
    }

    fn flip_over_x(&self) -> Image {
        let mut raw_data = Vec::with_capacity(self.height * self.width);
        let reverser = self.height - 1;

        for y in 0..self.height {
            for x in 0..self.width {
                raw_data.push(self.get(x, reverser - y))
            }
        }

        Image {
            height: self.height,
            width: self.width,
            raw_data,
        }
    }

    fn flip_over_y(&self) -> Image {
        let mut raw_data = Vec::with_capacity(self.height * self.width);
        let reverser = self.width - 1;

        for y in 0..self.height {
            for x in 0..self.width {
                raw_data.push(self.get(reverser - x, y))
            }
        }

        Image {
            height: self.height,
            width: self.width,
            raw_data,
        }
    }

    fn rotate(&self) -> Image {
        let mut res = self.clone();

        for x in 0..self.width {
            for y in 0..self.height {
                res.set(self.width - 1 - y, x, self.get(x, y));
            }
        }

        res
    }

    fn variants(self) -> Vec<Image> {
        let mut res = vec![];
        let mut new_elements = vec![self];

        while !new_elements.is_empty() {
            res.append(&mut new_elements);

            for current in res.iter() {
                let flip_x = current.flip_over_x();
                if !res.contains(&flip_x) && !new_elements.contains(&flip_x) {
                    new_elements.push(flip_x);
                }

                let flip_y = current.flip_over_y();
                if !res.contains(&flip_y) && !new_elements.contains(&flip_y) {
                    new_elements.push(flip_y);
                }

                let rotated = current.rotate();
                if !res.contains(&rotated) && !new_elements.contains(&rotated) {
                    new_elements.push(rotated);
                }
            }
        }

        res
    }

    fn blocks(&self) -> usize {
        self.raw_data.iter().filter(|x| x == &&Pixel::Block).count()
    }
}

fn main() -> Result<(), TopLevelError> {
    let filename = env::args().nth(1).expect("No file argument given.");
    let contents = fs::read_to_string(filename)?;
    let mut lines = contents.lines();
    let mut tiles = Vec::new();
    let mut num_originals = 0;

    while let Some(new_tile) = Tile::read(&mut lines)? {
        let mut new_tiles = new_tile.variants();
        tiles.append(&mut new_tiles);
        num_originals += 1;
    }

    if num_originals == 0 {
        return Err(TopLevelError::NoInputFound);
    }

    let mut board = Board::new(num_originals, tiles);
    board.print_status();
    while board.reduce() {
        println!("---");
        board.print_status();
    }
    let final_value = board.solve()?;
    let tl = final_value.get(0, 0)[0].identity;
    let tr = final_value.get(final_value.edge_length - 1, 0)[0].identity;
    let bl = final_value.get(0, final_value.edge_length - 1)[0].identity;
    let br = final_value.get(final_value.edge_length - 1, final_value.edge_length - 1)[0].identity;

    println!();
    println!(
        "Part 1 result:{} * {} * {} * {} = {}",
        tl,
        tr,
        bl,
        br,
        tl * tr * bl * br
    );
    println!();
    let mut base_image = Image::from(final_value);
    base_image.draw();
    println!("---------------------");
    let sea_monster = Image::sea_monster();
    sea_monster.draw();
    println!("---------------------");
    for image in base_image.variants().iter_mut() {
        if image.overlay(&sea_monster) > 0 {
            image.draw();
            println!("Blocks left: {}", image.blocks());
        }
    }
    Ok(())
}
