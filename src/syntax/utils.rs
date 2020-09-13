use std::collections::VecDeque;
use std::iter::FromIterator;

#[inline]
pub fn escape_char(i: char) -> char {
    match i {
        '\\' => '\\',
        '\"' => '\"',
        '\'' => '\'',
        'n' => '\n',
        'r' => '\r',
        't' => '\t',
        _ => unreachable!(),
    }
}

#[inline]
pub fn state_machine(
    (mut prev, mut is_escape): (VecDeque<char>, bool),
    item: char,
) -> (VecDeque<char>, bool) {
    if is_escape {
        prev.push_back(escape_char(item));
        is_escape = false;
    } else {
        if item == '\\' {
            is_escape = true;
        } else {
            prev.push_back(item);
            is_escape = false;
        }
    }
    (prev, is_escape)
}

#[inline]
pub fn escape_str(i: &str) -> String {
    let (char_string, is_escape) = i.chars().fold((VecDeque::new(), false), state_machine);
    assert_eq!(is_escape, false);
    String::from_iter(char_string.iter())
}

#[inline]
pub fn str2char(i: &str) -> char {
    i.chars().nth(0).unwrap()
}

#[inline]
pub fn register_intern_str(i: &str) -> String {
    // FIXME
    i.to_string()
}
