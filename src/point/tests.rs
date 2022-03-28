use super::*;

#[test]
fn test_col_name() {
    assert_eq!(Point::new(0, 1).column_name(), "B");
}

#[test]
fn test_col_name_wrap() {
    assert_eq!(Point::new(0, 701).column_name(), "ZZ");
}

#[test]
fn test_point_display() {
    assert_eq!(format!("{}", Point::new(0, 0)), "A1");
    assert_eq!(format!("{}", Point::new(5, 701)), "ZZ6");
}