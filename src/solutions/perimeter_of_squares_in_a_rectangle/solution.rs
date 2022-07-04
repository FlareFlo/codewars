fn perimeter(n: u64) -> u64 {
	let mut last = 0;
	let mut current = 1;
	let mut sum = 1;

	for _ in 0..n {
		let result = last + current;
		last = current;
		current = result;
		sum += result;
	}
	sum * 4
}


fn dotest(n: u64, exp: u64) -> () {
	assert_eq!(perimeter(n), exp)
}

#[test]
fn basics_perimeter() {
	dotest(5, 80);
	dotest(7, 216);
	dotest(20, 114624);
	dotest(30, 14098308);
}