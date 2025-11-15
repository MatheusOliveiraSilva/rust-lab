use fundamentals::lifetimes::{take_slice};

#[test]
fn test_basic_take_slice() {
    let s = "Hello word!";
    let (start, end) = (0, 3);
    let slice = take_slice(s, start, end).unwrap();
    assert_eq!(slice, "Hel");
}

#[test]
fn test_mid_take_slice() {
    let s = "Hello word!";
    let (start, end) = (3, 9);
    let slice = take_slice(s, start, end).unwrap();
    assert_eq!(slice, "lo wor");
}

#[test]
fn test_start_gt_end_take_slice() {
    let s = "Hello word!";
    let (start, end) = (9, 3); // start > end -> None
    let result = take_slice(s, start, end);
    assert_eq!(result, None);
}

#[test]
fn test_out_of_index_take_slice() {
    let s = "Hello word!";
    let (start, end) = (5, 25); // end > s.len() -> None
    let result = take_slice(s, start, end);
    assert_eq!(result, None);
}
