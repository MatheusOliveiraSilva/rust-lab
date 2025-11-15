use fundamentals::lifetimes::{take_slice, select_range, split_once};

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

#[test]
fn test_select_range() {
    let buffer = "Hello word!";
    let (start, end) = (0, 5);
    let selection = select_range(buffer, start, end).unwrap();
    assert_eq!(selection, "Hello");

    let (start, end) = (5, 0);
    let selection = select_range(buffer, start, end).unwrap();
    assert_eq!(selection, "Hello");

    let (start, end) = (2, 5);
    let selection = select_range(buffer, start, end).unwrap();
    assert_eq!(selection, "llo");

    let (start, end) = (5, 2);
    let selection = select_range(buffer, start, end).unwrap();
    assert_eq!(selection, "llo");

    let (start, end) = (2, 2);
    let result = select_range(buffer, start, end);
    assert_eq!(result, None);

    let (start, end) = (2, 25);
    let result = select_range(buffer, start, end);
    assert_eq!(result, None);

    let (start, end) = (25, 2);
    let result = select_range(buffer, start, end);
    assert_eq!(result, None);
}

#[test]
fn test_split_once() {
    let input = "a=b";
    let delimiter = '=';
    let (part1, part2) = split_once(input, delimiter).unwrap();
    assert_eq!(part1, "a");
    assert_eq!(part2, "b");

    let input = "=abc";
    let delimiter = '=';
    let (part1, part2) = split_once(input, delimiter).unwrap();
    assert_eq!(part1, "");
    assert_eq!(part2, "abc");

    let input = "abc=";
    let delimiter = '=';
    let (part1, part2) = split_once(input, delimiter).unwrap();
    assert_eq!(part1, "abc");
    assert_eq!(part2, "");

    let input = "abc";
    let delimiter = '=';
    let result = split_once(input, delimiter);
    assert_eq!(result, None);

    let input = "á=é";
    let delimiter = '=';
    let (part1, part2) = split_once(input, delimiter).unwrap();
    assert_eq!(part1, "á");
    assert_eq!(part2, "é");

    let input = "a=b=c";
    let delimiter = '=';
    let (part1, part2) = split_once(input, delimiter).unwrap();
    assert_eq!(part1, "a");
    assert_eq!(part2, "b=c");
}
