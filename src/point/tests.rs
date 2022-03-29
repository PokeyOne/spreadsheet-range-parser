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

#[test]
fn test_column_name_to_index() -> Result<(), String> {
    assert_eq!(Point::column_name_to_index("A")?, Some(0));
    assert_eq!(Point::column_name_to_index("B")?, Some(1));
    assert_eq!(Point::column_name_to_index("ZZ")?, Some(701));
    assert_eq!(Point::column_name_to_index("AAA")?, Some(702));
    Ok(())
}

#[test]
fn test_point_from_str() -> Result<(), String> {
    assert_eq!(Point::from_str("A1")?, Point::new(0, 0));
    assert_eq!(Point::from_str("ZZ6")?, Point::new(5, 701));
    assert_eq!(Point::from_str("AAA7")?, Point::new(6, 702));
    Ok(())
}
