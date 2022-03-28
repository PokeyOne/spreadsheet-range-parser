use super::*;

#[test]
fn test_from_str() {
    let r = Rectangle::from_str("A1:B2");
    assert_eq!(r.unwrap(), Rectangle {
        top_left: Point::new(0, 0),
        bottom_right: Point::new(1, 1),
    });
}
