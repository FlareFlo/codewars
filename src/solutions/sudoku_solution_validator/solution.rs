fn valid_solution(sudoku: &[[u8;9]; 9]) -> bool {
	// Validates an array to contain all require numbers, in any order
	let validate_collection =  |collection: &[u8]| {
		let mut state = true;
		for i in 1..=9_u8 {
			if !collection.contains(&i) {
				state = false;
				break;
			}
		}
		state
	};


	// validate rows
	for row in sudoku {
		if !validate_collection(row) {
			return false;
		}
	}

	// validate columns
	// rotate matrix clockwise first
	let mut rotated = vec![vec![0; 9];9];
	for i in 0..9 {
		for j in 0..9 {
			rotated[i][j] = sudoku[9 - j - 1][i];
		}
	}
	for column in rotated {
		if !validate_collection(&column) {
			return false;
		}
	}

	for x in 0..2 {
		for y in 0..2 {
			let x_offset = 3 * x;
			let y_offset = 3 * y;
			let mut subgrid = vec![];
			for i in 0..=2 {
				for j in 0..=2 {
					subgrid.push(sudoku[i + x_offset][j + y_offset])
				}
			}
			if !validate_collection(&subgrid) {
				return false;
			}
		}
	}
	true
}




#[cfg(test)]
mod sample_tests {
	use super::*;


	#[test]
	fn valid_sudoku() {
		let puzzle = [
			[7, 6, 9, 5, 3, 8, 1, 2, 4],
			[2, 4, 3, 7, 1, 9, 6, 5, 8],
			[8, 5, 1, 4, 6, 2, 9, 7, 3],
			[4, 8, 6, 9, 7, 5, 3, 1, 2],
			[5, 3, 7, 6, 2, 1, 4, 8, 9],
			[1, 9, 2, 8, 4, 3, 7, 6, 5],
			[6, 1, 8, 3, 5, 4, 2, 9, 7],
			[9, 7, 4, 2, 8, 6, 5, 3, 1],
			[3, 2, 5, 1, 9, 7, 8, 4, 6]];
		let actual = valid_solution(&puzzle);
		assert_eq!(actual, true, "\nYour result (left) did not match expected result (right).");
	}

	#[test]
	fn invalid_sudoku() {
		let puzzle = [[7, 6, 9, 5, 3, 8, 1, 2, 4],
			[2, 4, 3, 7, 1, 9, 6, 5, 8],
			[8, 5, 1, 4, 6, 2, 9, 7, 3],
			[4, 8, 6, 9, 7, 5, 3, 1, 2],
			[5, 3, 7, 6, 2, 1, 4, 8, 9],
			[1, 9, 2, 8, 4, 3, 7, 6, 5],
			[6, 1, 8, 3, 5, 4, 2, 9, 7],
			[9, 7, 4, 2, 8, 6, 5, 3, 1],
			[3, 2, 5, 1, 9, 7, 8, 4, 9]];
		let actual = valid_solution(&puzzle);
		assert_eq!(actual, false, "\nYour result (left) did not match expected result (right).");
	}

	#[test]
	fn invalid_with_zeroes() {
		let puzzle = [[3, 1, 5, 8, 4, 7, 6, 2, 9],
			[4, 7, 8, 2, 9, 6, 3, 5, 0],
			[2, 9, 6, 3, 5, 1, 7, 8, 4],
			[7, 4, 2, 9, 6, 8, 5, 1, 3],
			[6, 8, 9, 5, 1, 3, 4, 7, 2],
			[5, 0, 1, 4, 7, 2, 8, 9, 6],
			[1, 2, 4, 6, 8, 5, 9, 3, 7],
			[8, 6, 3, 7, 2, 9, 0, 4, 5],
			[9, 5, 7, 1, 3, 4, 2, 6, 8]];
		let actual = valid_solution(&puzzle);
		assert_eq!(actual, false, "\nYour result (left) did not match expected result (right).");
	}
}