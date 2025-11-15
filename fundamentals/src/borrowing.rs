// Basic borrow logics

pub fn min_max(v: &[i32]) -> Option<(&i32, &i32)> {
    if v.is_empty() {
        return None;
    }

    let mut min = &v[0];
    let mut max = &v[0];

    for slot in &v[1..] {
        if slot > max {
            max = slot;
        }
        if slot < min {
            min = slot
        }
    }

    return Some((min, max));
}
