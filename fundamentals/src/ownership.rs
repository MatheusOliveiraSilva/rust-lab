// Ownership studying
//
// Testing multiple ways to pass String as input.

pub fn append_owned(mut s: String, suffix: &str) -> String {
    s.push_str(suffix);
    s
}

pub fn append_borrow(s: &String, suffix: &str) -> String {
    let mut new_s = s.clone();
    new_s.push_str(suffix);
    new_s
}

pub fn append_in_place(s: &mut String, suffix: &str) {
    s.push_str(suffix);
}

pub fn take_and_consume(s: String) -> usize {
    s.len()
}
