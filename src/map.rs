use crate::errors::{MapOperationError, MapParseError};
use std::convert::{Into, TryFrom};

#[derive(Clone)]
pub struct Map<A: Clone> {
    width: usize,
    height: usize,
    data: Vec<Vec<A>>,
}

impl<'a, E, X> TryFrom<&'a str> for Map<X>
where
    X: Clone + TryFrom<char, Error = E>,
    E: From<MapParseError>,
{
    type Error = E;

    fn try_from(s: &str) -> Result<Map<X>, E> {
        let mut width = 0;
        let mut height = 0;
        let mut data = Vec::new();

        for line in s.lines() {
            let mut current_line = Vec::with_capacity(width);

            for char in line.chars() {
                let item = X::try_from(char)?;
                current_line.push(item);
            }

            height += 1;
            if width == 0 {
                width = current_line.len();
            } else if width != current_line.len() {
                return Err(E::from(MapParseError::UnevenLines(height)));
            }
            data.push(current_line);
        }

        Ok(Map {
            width,
            height,
            data,
        })
    }
}

impl<X: Clone> Map<X> {
    pub fn at(&self, x: usize, y: usize) -> Option<X> {
        let row = self.data.get(y)?;
        let wrapped_x = x % self.width;
        Some(row[wrapped_x].clone())
    }

    pub fn at_unwrapped(&self, x: usize, y: usize) -> Option<X> {
        let row = self.data.get(y)?;
        row.get(x).map(|x| x.clone())
    }

    fn view(&self, f: fn(&X) -> bool, x: usize, y: usize, rise: isize, run: isize) -> Option<X> {
        let mut sx = (x as isize) + run;
        let mut sy = (y as isize) + rise;

        loop {
            if sx < 0 { return None; }
            if sy < 0 { return None; }

            let entry = self.at_unwrapped(sx as usize, sy as usize)?;
            if f(&entry) {
                return Some(entry)
            }

            sx += run;
            sy += rise;
        }
    }

    pub fn set(&mut self, x: usize, y: usize, value: X) -> Result<(), MapOperationError> {
        let row = self.data.get_mut(y).ok_or(MapOperationError::OutOfBounds(x, y))?;

        if x >= self.width {
            return Err(MapOperationError::OutOfBounds(x, y));
        }

        row[x] = value;

        Ok(())
    }

    pub fn adjacents(&self, x: usize, y: usize) -> Result<Vec<X>, MapOperationError> {
        self.adjacents_until(x, y, |_| true)
    }

    pub fn adjacents_until(&self, x: usize, y: usize, f: fn(&X) -> bool) -> Result<Vec<X>, MapOperationError>
    {
        if y >= self.height {
            return Err(MapOperationError::OutOfBounds(x, y));
        }
        
        if x >= self.width {
            return Err(MapOperationError::OutOfBounds(x, y));
        }

        let mut results = Vec::new();
        push_some(&mut results, self.view(f, x, y, -1, -1));
        push_some(&mut results, self.view(f, x, y, -1, -0));
        push_some(&mut results, self.view(f, x, y, -1, 1));
        push_some(&mut results, self.view(f, x, y, -0, -1));
        push_some(&mut results, self.view(f, x, y, -0, 1));
        push_some(&mut results, self.view(f, x, y, 1, -1));
        push_some(&mut results, self.view(f, x, y, 1, 0));
        push_some(&mut results, self.view(f, x, y, 1, 1));


        Ok(results)
    }

    pub fn locations<'a>(&'a self) -> MapLocations<'a, X> {
        MapLocations {
            underlying: self,
            x: 0,
            y: 0,
        }
    }
}

impl<X: Clone + PartialEq> Map<X> {
    pub fn count(&self, x: X) -> usize {
        self.locations().filter(|v| v.2 == x).count()
    }
}

pub struct MapLocations<'a,X: Clone> {
    underlying: &'a Map<X>,
    x: usize,
    y: usize,
}

impl<'a, X: Clone> Iterator for MapLocations<'a,X> {
    type Item = (usize, usize, X);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if (self.x == self.underlying.width) && (self.y == self.underlying.height) {
                return None;
            }

            if self.x == self.underlying.width {
                self.x = 0;
                self.y += 1;
                continue;
            }

            let value = self.underlying.at_unwrapped(self.x, self.y)?;
            let result = (self.x, self.y, value);
            self.x += 1;

            return Some(result);
        }
    }
}

impl<X: Clone + Into<char>> Map<X> {
    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.at(x, y).unwrap().into());
            }
            println!("");
        }
    }
}

impl<X: Clone + PartialEq> PartialEq<Map<X>> for Map<X> {
    fn eq(&self, other: &Self) -> bool {
        self.height == other.height &&
          self.width == other.width &&
          self.data == other.data
    }
}

impl<X: Clone + PartialEq + Eq> Eq for Map<X> { }

fn push_some<X>(vector: &mut Vec<X>, value: Option<X>) {
    if let Some(val) = value {
        vector.push(val);
    }
}