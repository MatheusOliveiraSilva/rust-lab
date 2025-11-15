use fundamentals::borrowing::{min_max};

#[test]
fn test_min_max() {
    let v = vec![1, 5, 2, 10, 4];
    let (min, max) = min_max(&v).unwrap();
    assert_eq!(*min, 1);
    assert_eq!(*max, 10);
}

#[test]
fn test_negative_inputs_min_max() {
    let v = vec![-1, -10, -2, -5];
    let (min, max) = min_max(&v).unwrap();
    assert_eq!(*min, -10);
    assert_eq!(*max, -1);
}

#[test]
fn test_one_element_min_max() {
    let v = vec![1];
    let (min, max) = min_max(&v).unwrap();
    assert_eq!(*min, 1);
    assert_eq!(*max, 1);
}

#[test]
fn test_empty_min_max() {
    let v = vec![];
    let result = min_max(&v);
    assert_eq!(result, None);
}
