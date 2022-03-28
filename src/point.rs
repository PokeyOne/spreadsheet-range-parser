#[cfg(test)]
mod tests;

use std::fmt::Display;
use std::str::FromStr;

/// A specific cell in a spreadsheet.
///
/// For example, A1 is the top-left cell and would be represented as
/// `Point::new(0, 0)`. B2 would be `Point::new(1, 1)`.
#[derive(Debug, PartialEq, Clone)]
pub struct Point {
    /// The row **index** of the cell.
    pub row: usize,
    /// The column **index** of the cell.
    pub col: usize
}

impl Point {
    pub fn new(row: usize, col: usize) -> Point {
        Point { row, col }
    }

    /// Convert the index of the column into a letter representation.
    ///
    /// # Examples
    /// ```
    /// use spreadsheet_range_parser::point::Point;
    ///
    /// assert_eq!(Point::new(0, 0).column_name(), "A");
    /// assert_eq!(Point::new(0, 1).column_name(), "B");
    /// assert_eq!(Point::new(0, 25).column_name(), "Z");
    /// assert_eq!(Point::new(0, 26).column_name(), "AA");
    /// assert_eq!(Point::new(0, 27).column_name(), "AB");
    /// assert_eq!(Point::new(0, 28).column_name(), "AC");
    /// assert_eq!(Point::new(0, 701).column_name(), "ZZ");
    /// assert_eq!(Point::new(0, 702).column_name(), "AAA");
    /// assert_eq!(Point::new(0, 703).column_name(), "AAB");
    /// assert_eq!(Point::new(0, 18277).column_name(), "ZZZ");
    /// ```
    pub fn column_name(&self) -> String {
        let mut low_place = ((self.col % 26) as u8 + b'A') as char;

        let mut stack = Vec::new();
        stack.push(low_place);

        let mut col = self.col / 26;
        while col > 0 {
            let high_place = match col % 26 {
                0 => b'Z',
                _ => ((col - 1) % 26) as u8 + b'A'
            } as char;
            stack.push(high_place);

            col -= (col - 1) % 26;
            col /= 26;
        }

        stack.reverse();
        stack.into_iter().collect()
    }

    pub fn column_name_to_index(name: &str) -> Result<usize, String> {
        let mut value = 0;

        let mut chars = name.chars().rev();
        if let Some(first_char) = chars.next() {
            value += first_char as usize - 'A' as usize;
        } else {
            return Err("Empty column name".to_string());
        }

        let mut current_place = 1;
        for c in chars {
            value += ((c as usize - 'A' as usize) + 1) * 26_usize.pow(current_place);
            current_place += 1;
        }

        Ok(value)
    }
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Point, String> {
        let mut chars = s.chars().peekable();

        let mut column_name = String::new();
        while let Some(c) = chars.peek().copied() {
            if c.is_ascii_uppercase() {
                column_name.push(c);
            } else if c.is_ascii_lowercase() {
                column_name.push(c.to_ascii_uppercase());
            } else {
                break;
            }
            chars.next();
        }
        let column_index = Point::column_name_to_index(&column_name)?;

        let row = chars
            .collect::<String>()
            .parse::<usize>()
            .map_err(|e| format!("{e}"))?;

        if row == 0 {
            return Err("Row cannot be 0".to_string());
        }

        Ok(Point::new(row - 1, column_index))
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}{}", self.column_name(), self.row + 1)
    }
}
