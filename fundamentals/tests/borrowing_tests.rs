use fundamentals::borrowing::{min_max};

#[test]
fn test_min_max() {
    let v = vec![1, 5, 2, 10, 4];
    let (min, max) = min_max(&v).unwrap();
    assert_eq!(*min, 1);
    assert_eq!(*max, 10);
}
