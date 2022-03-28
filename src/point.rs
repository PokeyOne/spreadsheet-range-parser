#[cfg(test)]
mod tests;

use std::fmt::Display;

pub struct Point {
    pub row: usize,
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
                _ => ((col-1) % 26) as u8 + b'A'
            } as char;
            stack.push(high_place);

            col -= (col-1) % 26;
            col /= 26;
        }

        stack.reverse();
        stack.into_iter().collect()
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}{}", self.column_name(), self.row + 1)
    }
}
