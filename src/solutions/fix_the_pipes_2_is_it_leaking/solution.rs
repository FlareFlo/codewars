use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::ops::Index;

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Pipe {
	UP_AND_RIGHT,
	DOWN_AND_LEFT,
	DOWN_AND_RIGHT,
	UP_AND_LEFT,
	HORIZONTAL,
	VERTICAL,
	VERTICAL_AND_RIGHT,
	VERTICAL_AND_LEFT,
	DOWN_AND_HORIZONTAL,
	UP_AN_HORIZONTAL,
	VERTICAL_AND_HORIZONTAL,
	EMPTY,
}

impl Display for Pipe {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", match self {
			Pipe::UP_AND_RIGHT => { '┗' }
			Pipe::DOWN_AND_LEFT => { '┓' }
			Pipe::DOWN_AND_RIGHT => { '┏' }
			Pipe::UP_AND_LEFT => { '┛' }
			Pipe::HORIZONTAL => { '━' }
			Pipe::VERTICAL => { '┃' }
			Pipe::VERTICAL_AND_RIGHT => { '┣' }
			Pipe::VERTICAL_AND_LEFT => { '┫' }
			Pipe::DOWN_AND_HORIZONTAL => { '┳' }
			Pipe::UP_AN_HORIZONTAL => { '┻' }
			Pipe::VERTICAL_AND_HORIZONTAL => { '╋' }
			Pipe::EMPTY => { '.' }
		})
	}
}

impl Pipe {
	pub fn open_top(self) -> bool {
		match self {
			Pipe::UP_AND_RIGHT |
			Pipe::UP_AND_LEFT |
			Pipe::VERTICAL |
			Pipe::UP_AN_HORIZONTAL |
			Pipe::VERTICAL_AND_HORIZONTAL |
			Pipe::VERTICAL_AND_RIGHT |
			Pipe::VERTICAL_AND_LEFT => true,
			_ => { false }
		}
	}
	pub fn open_bottom(self) -> bool {
		match self {
			Pipe::DOWN_AND_LEFT |
			Pipe::DOWN_AND_RIGHT |
			Pipe::VERTICAL |
			Pipe::DOWN_AND_HORIZONTAL |
			Pipe::VERTICAL_AND_HORIZONTAL |
			Pipe::VERTICAL_AND_RIGHT |
			Pipe::VERTICAL_AND_LEFT => true,
			_ => { false }
		}
	}
	pub fn open_left(self) -> bool {
		match self {
			Pipe::DOWN_AND_LEFT |
			Pipe::UP_AND_LEFT |
			Pipe::HORIZONTAL |
			Pipe::VERTICAL_AND_LEFT |
			Pipe::DOWN_AND_HORIZONTAL |
			Pipe::UP_AN_HORIZONTAL |
			Pipe::VERTICAL_AND_HORIZONTAL => true,
			_ => { false }
		}
	}
	pub fn open_right(self) -> bool {
		match self {
			Pipe::UP_AND_RIGHT |
			Pipe::DOWN_AND_RIGHT |
			Pipe::HORIZONTAL |
			Pipe::VERTICAL_AND_RIGHT |
			Pipe::DOWN_AND_HORIZONTAL |
			Pipe::UP_AN_HORIZONTAL |
			Pipe::VERTICAL_AND_HORIZONTAL => true,
			_ => { false }
		}
	}
}

impl From<char> for Pipe {
	fn from(value: char) -> Self {
		match value {
			'┗' => Pipe::UP_AND_RIGHT,
			'┓' => Pipe::DOWN_AND_LEFT,
			'┏' => Pipe::DOWN_AND_RIGHT,
			'┛' => Pipe::UP_AND_LEFT,
			'━' => Pipe::HORIZONTAL,
			'┃' => Pipe::VERTICAL,
			'┣' => Pipe::VERTICAL_AND_RIGHT,
			'┫' => Pipe::VERTICAL_AND_LEFT,
			'┳' => Pipe::DOWN_AND_HORIZONTAL,
			'┻' => Pipe::UP_AN_HORIZONTAL,
			'╋' => Pipe::VERTICAL_AND_HORIZONTAL,
			'.' => Pipe::EMPTY,
			_ => { panic!("Unknown char {value}") }
		}
	}
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash, Debug)]
struct Coordinate(i32, i32);

impl Display for Coordinate {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "(x: {}, y: {})", self.0, self.1)
	}
}

struct CharMap {
	map: Vec<Vec<Pipe>>,
	pub size_x: i32,
	pub size_y: i32,
}

impl CharMap {
	pub fn new(map: &[&str]) -> Self {
		Self {
			map: map.iter().map(|e| e.chars().map(|e| Pipe::from(e)).collect()).collect(),
			size_x: map[0].chars().count() as _,
			size_y: map.len() as _,
		}
	}

	pub fn get_sources(&self) -> Vec<Coordinate> {
		let mut out = HashSet::new();

		// Top Row - Left to right
		for x in 0..self.size_x {
			let coords = Coordinate(x as _, 0);
			let char = self[coords];
			if char.open_top() {
				out.insert(coords);
			}
		}

		// Bottom Row - Left to right
		for x in 0..self.size_x {
			let coords = Coordinate(x as _, self.size_y - 1);
			let char = self[coords];
			if char.open_bottom() {
				out.insert(coords);
			}
		}

		// Left column - Top to bottom
		for y in 0..self.size_y {
			let coords = Coordinate(0, y as _);
			let char = self[coords];
			if char.open_left() {
				out.insert(coords);
			}
		}

		// Right column - Top to bottom
		for y in 0..self.size_y {
			let coords = Coordinate(self.size_x - 1, y as _);
			let char = self[coords];
			if char.open_right() {
				out.insert(coords);
			}
		}

		out.into_iter().collect()
	}

	pub fn in_bounds(&self, coords: Coordinate) -> bool {
		self.size_x > coords.0 && self.size_y > coords.1 && coords.0 >= 0 && coords.1 >= 0
	}

	/// Block bordering the map
	pub fn is_bounding(&self, coords: Coordinate) -> bool {
		(coords.0 == self.size_x || coords.0 == -1) || (coords.1 == self.size_y || coords.1 == -1)
	}

	pub fn debug(&self) {
		for pip in &self.map {
			println!("{}", pip.iter().map(|e| e.to_string()).collect::<String>());
		}
	}

	pub fn is_leaking_nearby(&self, coord: Coordinate) -> bool {
		let pipe = self[coord];

		if pipe.open_top() {
			let neighbor = Coordinate(coord.0, coord.1 - 1);
			if !self.is_bounding(neighbor) && !self[neighbor].open_bottom() {
				return true;
			}
		}

		if pipe.open_bottom() {
			let neighbor = Coordinate(coord.0, coord.1 + 1);
			if !self.is_bounding(neighbor) && !self[neighbor].open_top() {
				return true;
			}
		}

		if pipe.open_left() {
			let neighbor = Coordinate(coord.0 - 1, coord.1);
			if !self.is_bounding(neighbor) && !self[neighbor].open_right() {
				return true;
			}
		}

		if pipe.open_right() {
			let neighbor = Coordinate(coord.0 + 1, coord.1);
			if !self.is_bounding(neighbor) && !self[neighbor].open_left() {
				return true;
			}
		}

		false
	}
}

impl Index<Coordinate> for CharMap {
	type Output = Pipe;

	fn index(&self, index: Coordinate) -> &Self::Output {
		if !self.in_bounds(index) {
			panic!("Index {index} out of bounds for size {}x{}", self.size_x, self.size_y)
		}
		&self.map[index.1 as usize][index.0 as usize]
	}
}


fn check_pipe(pipe_map: &[&str]) -> bool {
	let map = CharMap::new(pipe_map);
	let sources = map.get_sources();
	sources.into_iter().all(|source| !source_leaks_water(&map, HashSet::new(), source))
}

// Starts with a list of already seen nodes and a source block that carries water
// The visited list is used to backtrack and return when reaching known terrain (circles etc.)
fn source_leaks_water(map: &CharMap, mut visited: HashSet<Coordinate>, source_block: Coordinate) -> bool {
	// If the source leaks directly nearby we can return early
	if map.is_leaking_nearby(source_block) {
		return true;
	}

	visited.insert(source_block);
	let current = map[source_block];

	if current.open_top() {
		let next = Coordinate(source_block.0, source_block.1 - 1);
		if !map.is_bounding(next) && !visited.contains(&next) {
			if source_leaks_water(map, visited.clone(), next) {
				return true;
			}
		}
	}

	if current.open_bottom() {
		let next = Coordinate(source_block.0, source_block.1 + 1);
		if !map.is_bounding(next) && !visited.contains(&next) {
			if source_leaks_water(map, visited.clone(), next) {
				return true;
			}
		}
	}

	if current.open_left() {
		let next = Coordinate(source_block.0 - 1, source_block.1);
		if !map.is_bounding(next) && !visited.contains(&next) {
			if source_leaks_water(map, visited.clone(), next) {
				return true;
			}
		}
	}

	if current.open_right() {
		let next = Coordinate(source_block.0 + 1, source_block.1);
		if !map.is_bounding(next) && !visited.contains(&next) {
			if source_leaks_water(map, visited.clone(), next) {
				return true;
			}
		}
	}

	false
}


fn run_test(pmap: &[&str], answer: bool) {
	let test_result = check_pipe(pmap);
	assert!(
		test_result == answer,
		"Output: {}; expected value: {}; for input:\n{}\n",
		test_result,
		answer,
		pmap.join("\n")
	);
}

#[cfg(test)]
mod sample_tests {
	#[test]
	fn small_fixed_tests() {
		for (pmap, answer) in &TEST_CASES {
			super::run_test(pmap, *answer);
		}
	}

	const BIG: &[&str] = &[
		"╋━┛┃.┗━┻┛...",
		"┻━━┛........",
		"............",
		"..┏┳━┓......",
		"━┓┃┗┓┃......",
		"━┻┛.┫┛......",
		".........┏━━",
		".........┃..",
		"...┏┓....┗━┓",
		"...┣┛......┗",
		"...┗━━┓...┏━",
		"......┃...┃.",
		".....┏┛...┃.",
		".┏┓..┃....┗━",
	];

	const TEST_CASES: [(&[&str], bool); 8] = [
		(BIG, false),
		(&["╋━━┓", "┃..┃", "┛..┣"], true),
		(&["...┏", "┃..┃", "┛..┣"], false),
		(&["...┏", "...┃", "┛..┣"], false),
		(&["...┏", "...┃", "┓..┣"], true),
		(&["╋", "╋", "╋"], true),
		(&["╋....", "┃..┛.", "┃...."], false),
		(&["....", ".┛┛.", "...."], true),
	];
}