#[cfg(test)]
mod tests;

use crate::point::Point;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub struct Rectangle {
    pub top_left: Point,
    pub bottom_right: Point,
}

impl FromStr for Rectangle {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let no_spaces = s.replace(" ", "");
        let mut split = no_spaces.split(':');
        let top_left = split.next().ok_or("No start point")?;
        let bottom_right = split.next().ok_or("No end point")?;

        let top_left = Point::from_str(top_left)?;
        let bottom_right = Point::from_str(bottom_right)?;

        Ok(Rectangle { top_left, bottom_right })
    }
}
