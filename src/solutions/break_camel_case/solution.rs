fn solution(s: &str) -> String {
	let mut new_string: Vec<char> = Vec::new();
	let old_string = s.chars().collect::<Vec<char>>();

	for char in old_string {
		if char.is_lowercase() {
			new_string.push(char);
		} else {
			new_string.push(' ');
			new_string.push(char);
		}
	}
	new_string.into_iter().collect()
}