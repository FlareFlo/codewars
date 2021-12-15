use std::str::FromStr;

fn descending_order(x: u64) -> u64 {
	let string = x.to_string();
	let mut chars = string.chars().collect::<Vec<char>>();
	chars.sort_by(|a, b| b.cmp(a));

	let string_char = chars.into_iter().collect::<String>();

	u64::from_str(&string_char).unwrap()
}