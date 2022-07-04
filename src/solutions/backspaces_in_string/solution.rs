fn clean_string(s: &str) -> String {
	let mut new = String::new();
	let mut counter = 0;
	for char in s.chars().rev() {
		if char == '#' {
			counter += 1;
		} else {
			if counter != 0 {
				counter -= 1;
			} else {
				new.push(char)
			}
		}
	}
	new.chars().rev().collect::<String>()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn sample_tests() {
		assert_eq!(clean_string("abc#d##c"), "ac");
		assert_eq!(clean_string("abc####d##c#"), "");
		assert_eq!(clean_string("#######"), "");
		assert_eq!(clean_string(""), "");
	}
}