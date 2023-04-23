
pub type Num = f64;
pub type Mat<'a> = &'a [Num];
pub type MatX2<'a> = Vec<Vec<Num>>;

fn determinant(matrix: &[Vec<i64>]) -> i64 {
	_determinant(matrix.iter().map(|x|x.iter().map(|y|*y as f64).collect()).collect()).round() as i64
}

fn _determinant(mut matrix: MatX2) -> Num {
	debug_mat(matrix.clone());

	let mut is_diagonal = false;
	let mut col = 0_usize;

	while !is_diagonal {

	}






	diagonal_determinant(matrix)
}


fn mult_row(mut mat: Vec<Vec<Num>>, target_row: usize, mul: Num) -> Vec<Vec<Num>> {

	for col in mat.iter_mut() {
		for row in col.iter_mut().enumerate() {
			if row.0 == target_row {
				*row.1 *= mul;
			}
		}
	}

	mat
}

fn diagonal_determinant(mat: MatX2) -> Num {
	let mut default = 1.0;

	for col in mat.iter().enumerate() {
		for row in col.1.iter().enumerate() {
			if col.0 == row.0 {
				default *= row.1;
			}
		}
	}
	default
}

fn debug_mat(mat: Vec<Vec<Num>>) {
	let mut rows = vec![String::new(); mat[0].len()];
	for (y,col) in mat.iter().enumerate() {
		for (x, row) in col.iter().enumerate() {
			rows[x].push_str(&(row.to_string() + " "));
		}
	}
	println!("{}", rows.join("\n"));
}


#[cfg(test)]
mod tests {
	use crate::solutions::matrix_determinant::solution::{debug_mat, mult_row};
	use super::determinant;

	const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

	fn dotest(a: &[Vec<i64>], expected: i64) {
		assert_eq!(determinant(a), expected, "{ERR_MSG}")
	}

	#[test]
	fn sample_tests() {
		// dotest(&[vec![1]], 1);
		// dotest(&[vec![1, 3],  vec![2,5]], -1);
		dotest(&[vec![2, 5, 3], vec![1, -2, -1], vec![1, 3, 4]], -20);
	}

	#[test]
	fn other_test() {
		let mut mat: Vec<Vec<f64>> = vec![vec![1.0,1.0,1.0,1.0,1.0], vec![1.0,1.0,1.0,1.0,1.0], vec![1.0,1.0,1.0,1.0,1.0],vec![1.0,1.0,1.0,1.0,1.0],vec![1.0,1.0,1.0,1.0,1.0]];
		let target = vec![vec![1.0,1.0,1.0,2.0,1.0], vec![1.0,1.0,1.0,2.0,1.0], vec![1.0,1.0,1.0,2.0,1.0],vec![1.0,1.0,1.0,2.0,1.0],vec![1.0,1.0,1.0,2.0,1.0]];
		mat = mult_row(mat, 3, 2.0);
		assert_eq!(mat, target);
	}
}
