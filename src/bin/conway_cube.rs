use advent2020::errors::{MapParseError, TopLevelError};
use std::collections::HashSet;
use std::env;
use std::fmt;
use std::fs;
use std::ops::RangeInclusive;
use std::str::FromStr;

struct ConwayCube {
    x_range: RangeInclusive<isize>,
    y_range: RangeInclusive<isize>,
    z_range: RangeInclusive<isize>,
    active_points: HashSet<(isize, isize, isize)>,
}

impl ConwayCube {
    fn is_active(&self, x: isize, y: isize, z: isize) -> bool {
        self.active_points.contains(&(x, y, z))
    }

    fn active_neighbors(&self, x: isize, y: isize, z: isize) -> usize {
        let mut count = 0;

        for x_offset in -1..=1 {
            for y_offset in -1..=1 {
                for z_offset in -1..=1 {
                    if (x_offset != 0) || (y_offset != 0) || (z_offset != 0) {
                        if self.is_active(x + x_offset, y + y_offset, z + z_offset) {
                            count += 1;
                        }
                    }
                }
            }
        }

        count
    }

    fn next(&self) -> ConwayCube {
        let x_range = (self.x_range.start() - 1)..=(self.x_range.end() + 1);
        let y_range = (self.y_range.start() - 1)..=(self.y_range.end() + 1);
        let z_range = (self.z_range.start() - 1)..=(self.z_range.end() + 1);
        let mut active_points = HashSet::new();

        for x in x_range.clone() {
            for y in y_range.clone() {
                for z in z_range.clone() {
                    let active_neighbors = self.active_neighbors(x, y, z);

                    if self.is_active(x, y, z) && (active_neighbors == 2 || active_neighbors == 3) {
                        active_points.insert((x, y, z));
                    } else {
                        if self.active_neighbors(x, y, z) == 3 {
                            active_points.insert((x, y, z));
                        }
                    }
                }
            }
        }

        ConwayCube { x_range, y_range, z_range, active_points }
    }
}

impl FromStr for ConwayCube {
    type Err = TopLevelError;

    fn from_str(contents: &str) -> Result<Self, Self::Err> {
        let mut computed_width = None;
        let mut data = Vec::new();

        for line in contents.lines() {
            let start_size = data.len();
            for char in line.chars() {
                match char {
                    '.' => data.push(false),
                    '#' => data.push(true),
                    _ => return Err(MapParseError::UnexpectedCharacter(char))?,
                }
            }

            match computed_width {
                None => computed_width = Some(data.len()),
                Some(first_width) if data.len() - start_size == first_width => {}
                Some(first_width) => return Err(MapParseError::UnevenLines(first_width))?,
            }
        }

        let width = computed_width.ok_or(TopLevelError::NoInputFound)? as isize;
        let height = data.len() as isize / width;

        if width != height {
            return Err(TopLevelError::MapParseError(MapParseError::UnevenLines(width as usize)));
        }

        let x_range = 0..=(width-1);
        let y_range = 0..=(height-1);
        let z_range = 0..=0;
        let mut active_points = HashSet::new();

        let mut data_points = data.iter();
        for y in y_range.clone() {
            for x in x_range.clone() {
                if *data_points.next().ok_or(TopLevelError::UnknownError)? {
                    active_points.insert((x, y, 0));
                }
            }
        }

        Ok(ConwayCube { x_range, y_range, z_range, active_points })
    }
}

impl fmt::Display for ConwayCube {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for z in self.z_range.clone() {
            write!(f, "z={}\n", z)?;
            for y in self.y_range.clone() {
                for x in self.x_range.clone() {
                    if self.is_active(x, y, z) {
                        write!(f, "#")?;
                    } else {
                        write!(f, ".")?;
                    }
                }
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}
struct Conway4Cube {
    x_range: RangeInclusive<isize>,
    y_range: RangeInclusive<isize>,
    z_range: RangeInclusive<isize>,
    w_range: RangeInclusive<isize>,
    active_points: HashSet<(isize, isize, isize, isize)>,
}

impl Conway4Cube {
    fn is_active(&self, x: isize, y: isize, z: isize, w: isize) -> bool {
        self.active_points.contains(&(x, y, z, w))
    }

    fn active_neighbors(&self, x: isize, y: isize, z: isize, w: isize) -> usize {
        let mut count = 0;

        for x_offset in -1..=1 {
            for y_offset in -1..=1 {
                for z_offset in -1..=1 {
                    for w_offset in -1..=1 {
                        if (x_offset != 0) || (y_offset != 0) || (z_offset != 0) || (w_offset != 0) {
                            if self.is_active(x + x_offset, y + y_offset, z + z_offset, w + w_offset) {
                                count += 1;
                            }
                        }
                    }
                }
            }
        }

        count
    }

    fn next(&self) -> Conway4Cube {
        let x_range = (self.x_range.start() - 1)..=(self.x_range.end() + 1);
        let y_range = (self.y_range.start() - 1)..=(self.y_range.end() + 1);
        let z_range = (self.z_range.start() - 1)..=(self.z_range.end() + 1);
        let w_range = (self.w_range.start() - 1)..=(self.w_range.end() + 1);

        let mut active_points = HashSet::new();

        for x in x_range.clone() {
            for y in y_range.clone() {
                for z in z_range.clone() {
                    for w in w_range.clone() {
                        let active_neighbors = self.active_neighbors(x, y, z, w);

                        if self.is_active(x, y, z, w) && (active_neighbors == 2 || active_neighbors == 3) {
                            active_points.insert((x, y, z, w));
                        } else {
                            if self.active_neighbors(x, y, z, w) == 3 {
                                active_points.insert((x, y, z, w));
                            }
                        }
                    }
                }
            }
        }

        Conway4Cube { x_range, y_range, z_range, w_range, active_points }
    }
}

impl<'a> From<&'a ConwayCube> for Conway4Cube {
    fn from(x: &ConwayCube) -> Conway4Cube {
        Conway4Cube {
            x_range: x.x_range.clone(),
            y_range: x.x_range.clone(),
            z_range: x.x_range.clone(),
            w_range: 0..=0,
            active_points: x.active_points.iter().map(|(x,y,z)| (*x, *y, *z, 0)).collect(),
        }
    }
}

fn main() -> Result<(), TopLevelError> {
    let filename = env::args().nth(1).expect("No file argument given.");
    let contents = fs::read_to_string(filename)?;
    let mut cube = ConwayCube::from_str(&contents)?;
    let mut cube4 = Conway4Cube::from(&cube);

    for _ in 0..6 {
        //println!("-------------------------------------------------");
        //println!("{}\n", cube);
        cube = cube.next();
        cube4 = cube4.next();
    }

    println!("Active 3-dimensional points in the end: {}", cube.active_points.len());
    println!("Active 4-dimensional points in the end: {}", cube4.active_points.len());

    Ok(())
}