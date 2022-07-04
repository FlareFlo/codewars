fn multiplication_table(len: usize) -> Vec<Vec<usize>> {
	let mut result: Vec<Vec<usize>> = vec![vec![1; len]; len];

	for x in 1..=len {
		for y in 1..=len {
			result[x - 1][y - 1] = x * y;
		}
	}
	result
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn basic() {
		assert_eq!(multiplication_table(3), [[1,2,3], [2,4,6], [3,6,9]]);
	}
}