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

#[test]
fn test_point_with_no_row() -> Result<(), String> {
    assert_eq!(Point::from_str("A")?, Point::new_opt(None, Some(0)));
    assert_eq!(Point::from_str("ZZ")?, Point::new_opt(None, Some(701)));
    assert_eq!(Point::from_str("AAA")?, Point::new_opt(None, Some(702)));
    Ok(())
}

#[test]
fn test_point_with_no_col() -> Result<(), String> {
    assert_eq!(Point::from_str("1")?, Point::new_opt(Some(0), None));
    assert_eq!(Point::from_str("701")?, Point::new_opt(Some(700), None));
    assert_eq!(Point::from_str("702")?, Point::new_opt(Some(701), None));
    Ok(())
}

#[test]
#[should_panic]
fn test_point_with_no_row_and_no_col() {
    Point::new_opt(None, None);
}