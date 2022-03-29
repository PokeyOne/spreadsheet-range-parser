//! # Spreadsheet Range Parser
//!
//! Parse spreadsheet range and cell indicators
//!
//! ## Usage
//!
//! Parsing a cell indicator:
//! ```rust
//! use spreadsheet_range_parser::point::Point;
//! use std::str::FromStr;
//!
//! // You can convert a string to a Point
//! assert_eq!(Point::from_str("A1"), Ok(Point::new(0, 0)));
//! assert_eq!(Point::from_str("B2"), Ok(Point::new(1, 1)));
//! assert_eq!(Point::from_str("ZZ3"), Ok(Point::new(2, 701)));
//!
//! // You can also convert a Point to a string
//! assert_eq!(Point::new(0, 0).to_string(), "A1");
//! assert_eq!(Point::new(1, 1).to_string(), "B2");
//! assert_eq!(Point::new(2, 701).to_string(), "ZZ3");
//! ```
//!
//! Parsing a range indicator:
//! ```rust
//! use spreadsheet_range_parser::rectangle::Rectangle;
//! use spreadsheet_range_parser::point::Point;
//! use std::str::FromStr;
//!
//! // You can convert a string to a Rectangle.
//! assert_eq!(Rectangle::from_str("A1:B2"), Ok(Rectangle::new(Point::new(0, 0), Point::new(1, 1))));
//! // You can also have infinite ranges.
//! assert_eq!(Rectangle::from_str("A1:C"), Ok(Rectangle::new(Point::new(0, 0), Point { row: None, col: Some(2) })));
//!
//! // You can also convert a Rectangle to a string
//! assert_eq!(Rectangle::new(Point::new(0, 0), Point::new(1, 1)).to_string(), "A1:B2");
//! ```

pub mod point;
pub mod rectangle;
