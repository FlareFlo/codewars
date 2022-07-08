fn rot13(message: &str) -> String {
	const OFFSET: usize = 13;
	let alphabet = (65..=90).map(|x| char::from(x)).collect::<Vec<char>>();
	let mut new = "".to_owned();
	for char in message.chars() {
		new.push(if char.is_ascii_alphabetic() {
			let new_char = alphabet[(char.to_ascii_uppercase() as usize - 65 + OFFSET) % 26];
			if char.is_uppercase() {
				new_char
			} else {
				new_char.to_ascii_lowercase()
			}
		} else {
			char
		})
	}
	new
}

#[cfg(test)]
mod tests {
	use super::rot13;

	const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

	fn dotest(s: &str, expected: &str) {
		assert_eq!(rot13(s), expected, "{ERR_MSG} with message = \"{s}\"")
	}

	#[test]
	fn sample_tests() {
		dotest("test", "grfg");
		dotest("Test", "Grfg");
	}
}