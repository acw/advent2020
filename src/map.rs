use crate::errors::MapParseError;
use std::convert::TryFrom;

pub struct Map<A: Clone> {
    width: usize,
    _height: usize,
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
        let mut _height = 0;
        let mut data = Vec::new();

        for line in s.lines() {
            let mut current_line = Vec::with_capacity(width);

            for char in line.chars() {
                let item = X::try_from(char)?;
                current_line.push(item);
            }

            _height += 1;
            if width == 0 {
                width = current_line.len();
            } else {
                if width != current_line.len() {
                    return Err(E::from(MapParseError::UnevenLines(_height)));
                }
            }
            data.push(current_line);
        }

        Ok(Map {
            width,
            _height,
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
}
