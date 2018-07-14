use self::ParseState::*;
use self::Character::*;

const LOWER_ALPHA: [char; 26] = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];
const UPPER_ALPHA: [char; 26] = ['A' ,'B' ,'C' ,'D' ,'E' ,'F' ,'G' ,'H' ,'I' ,'J' ,'K' ,'L' ,'M' ,'N' ,'O' ,'P' ,'Q' ,'R' ,'S' ,'T' ,'U' ,'V' ,'W' ,'X' ,'Y' ,'Z'];
const NUMBERS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Character {
	Equal(char),
	OneOf(Vec<char>),
	//Wildcard
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParseState {
	Normal,
	StartRange(Vec<char>, Option<char>),
	EndRange(Vec<char>, char)
}

fn get_chars(range: &[char], start: char, end: char) -> Vec<char> {
	let mut seen = false;
	let mut ret = vec![];
	for &c in range {
		if seen && c == end {
			ret.push(c);
			return ret;
		} else if seen {
			ret.push(c);
		} else if !seen && c == start {
			ret.push(c);
			seen = true;
		}
	}
	ret
}

pub fn match_pattern(pattern: &[Character], s: &str) -> bool {
	if s.len() < pattern.len() {
		return false;
	}
	'outer: for (p, c) in pattern.iter().zip(s.chars()) {
		match p {
			&Equal(expected) => {
				if expected != c {
					return false;
				}
			},
			&OneOf(ref expected) => {
				for expected in expected {
					if *expected == c {
						continue 'outer;
					}
				}
				return false;
			}
		}
	}
	true
}

pub fn parse_pattern(pattern: &str) -> Result<Vec<Character>, ()> {
	let mut state = Normal;
	let mut res: Vec<Character> = vec![];

	for c in pattern.chars() {
		match state.clone() {
			Normal => {
				if c == '[' {
					state = StartRange(vec![], None);
				} else if c.is_alphanumeric() {
					res.push(Equal(c));
				} else {
					return Err(());
				}
			},
			StartRange(mut acc, mut last) => {
				if c == ']' {
					if let Some(last) = last {
						acc.push(last);
					}
					res.push(OneOf(acc));
					state = Normal;
				} else if c.is_alphanumeric() {
					if let Some(last) = last {
						acc.push(last);
					}
					last = Some(c);
					state = StartRange(acc, last);
				} else if c == '-' {
					if last.is_none() {
						return Err(());
					}
					state = EndRange(acc, last.unwrap());
				} else {
					return Err(());
				}
			},
			EndRange(mut acc, mut last) => {
				if c.is_alphanumeric() {
					if NUMBERS.contains(&last) && NUMBERS.contains(&c) {
						acc.extend(get_chars(&NUMBERS, last, c));
					} else if LOWER_ALPHA.contains(&last) && LOWER_ALPHA.contains(&c) {
						acc.extend(get_chars(&LOWER_ALPHA, last, c));
					} else if UPPER_ALPHA.contains(&last) && UPPER_ALPHA.contains(&c) {
						acc.extend(get_chars(&UPPER_ALPHA, last, c));
					} else {
						return Err(());
					}
					state = StartRange(acc, None);
				} else {
					return Err(());
				}
			}
		}
	}
	if let Normal = state {
		Ok(res)
	} else {
		Err(())
	}
}

#[test]
fn not_test() {
	//panic!("{:?}", parse_pattern("1boat[0-9]"));
	let pattern = parse_pattern("1boat[0-9]").unwrap();
	assert!(match_pattern(&pattern, "1boat3"));
	assert!(!match_pattern(&pattern, "1boatz"));
	assert!(!match_pattern(&pattern, "1b"));
}