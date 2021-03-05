use std::collections::{HashMap, VecDeque};
use std::{iter::FromIterator, sync::Mutex};

use lazy_static::lazy_static;

use crate::{error::{RuntimeError, SyntaxMatchError}, values::{Handle, Symbol}};

lazy_static! {
    static ref GLOBAL_INTERN_STRING_POOL: Mutex<HashMap<Handle<String>, Handle<String>>> =
        Mutex::new(HashMap::new());
}

// fast(xD
#[macro_export]
macro_rules! fast_return {
	($e:expr) => {
		if let Ok(res) = $e {
			return Ok(res);
		}
	};
}

#[inline]
pub fn string_intern(i: &str) -> Handle<String> {
    let k = Handle::new(i.to_string());
    {
        if let Some(x) = GLOBAL_INTERN_STRING_POOL.lock().unwrap().get(&k) {
            return x.clone();
        }
    }
    GLOBAL_INTERN_STRING_POOL
        .lock()
        .unwrap()
        .insert(k.clone(), k.clone());
    k
}

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
        return (prev, false);
    }
    if item == '\\' {
        is_escape = true;
    } else {
        prev.push_back(item);
        is_escape = false;
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
    i.chars().next().unwrap()
}

#[inline]
pub fn match_error(keyword: &Handle<Symbol>) -> RuntimeError {
	RuntimeError::SyntaxError(
		SyntaxMatchError::SyntaxMatchError(keyword.clone()))
}