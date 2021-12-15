fn alphabet_position(text: &str) -> String {
	let chars = "abcdefghijklmnopqrstuvwxyz";

	let mut new_chars = Vec::new();
	for char in text.chars() {
		if let Some(pos) = chars.find(char.to_ascii_lowercase()) {
			new_chars.push(pos + 1);
		}
	}
	new_chars.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ")
}