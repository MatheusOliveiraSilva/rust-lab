use fundamentals::ownership::{
    append_owned, append_borrow, append_in_place, take_and_consume
};

/// Failure example
///
/// ```compile_fail
///let s = String::from("Hello");
///let result = append_owned(s, " world!");
///assert_eq!(result, "Hello world!");
///assert_eq!(s, "Hello"); // -> compilation error!
/// ```
#[test]
fn test_append_owned() {
    let s = String::from("Hello");
    let result = append_owned(s, " world!");
    assert_eq!(result, "Hello world!");
}

#[test]
fn test_append_borrow() {
    let s = String::from("Hello");
    let result = append_borrow(&s, " world!");
    assert_eq!(result, "Hello world!");
    assert_eq!(s, "Hello");
}

#[test]
fn test_append_in_place() {
    let mut s = String::from("Hello");
    append_in_place(&mut s, " world!");
    assert_eq!(s, "Hello world!");
}

/// Failure example
///
/// ```compile_fail
/// let s = String::from("Hello world!");
/// let len = take_and_consume(s);
/// assert_eq!(len, 12);
/// assert_eq!(s, "Hello world!") // -> value borrowed here after move
/// ```
#[test]
fn test_take_and_consume() {
    let s = String::from("Hello world!");
    let len = take_and_consume(s);
    assert_eq!(len, 12);
}
