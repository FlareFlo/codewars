use std::str::FromStr;

fn luck_check(ticket: &str) -> Option<bool> {
	let mut chars = vec![0; ticket.len()];
	for (i, char) in ticket.chars().enumerate() {
		if let Ok(num) = u8::from_str(&String::from(char)) {
			chars[i] = num;
		} else {
			return None;
		}
	}
	if chars.len() % 2 != 0 {
		chars.remove(chars.len() / 2);
	}
	Some(chars[..chars.len() / 2].iter().sum::<u8>() == chars[chars.len() / 2..].iter().sum::<u8>())
}

#[cfg(test)]
mod tests {
	use super::luck_check;

	const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

	fn dotest(s: &str, expected: Option<bool>) {
		assert_eq!(luck_check(s), expected, "{ERR_MSG} with ticket = \"{s}\"")
	}

	#[test]
	fn fixed_tests() {
		dotest("683179", Some(true));
		dotest("683000", Some(false));
		dotest("6F43E8", None);
		dotest("91856399083", Some(true))
	}
}
