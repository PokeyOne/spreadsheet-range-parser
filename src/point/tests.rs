use super::*;

#[test]
fn test_col_name() {
    assert_eq!(Point::new(0, 1).column_name(), "B");
}

#[test]
fn test_col_name_wrap() {
    assert_eq!(Point::new(0, 701).column_name(), "ZZ");
}