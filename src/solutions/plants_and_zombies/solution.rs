// TODO: Somewhere there is a sporadic error in here, no clue whats going on

use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum MapItem {
	Zombie {
		hp: usize,
	},
	SimpleShooter {
		count: usize,
	},
	SShooter,
	Empty,
}

impl MapItem {
	pub fn is_zombie(self) -> bool {
		match self {
			MapItem::Zombie { .. } => { true }
			_ => false
		}
	}
}

impl Display for MapItem {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", match self {
			MapItem::Zombie { hp } => { format!("{hp:02}") }
			MapItem::SimpleShooter { count } => { "S".to_string() + &count.to_string() }
			MapItem::SShooter => { "SX".to_string() }
			MapItem::Empty => { "--".to_string() }
		})
	}
}

impl From<char> for MapItem {
	fn from(value: char) -> Self {
		match value {
			' ' => Self::Empty,
			'S' => Self::SShooter,
			'1'..='9' => Self::SimpleShooter { count: usize::from_str(&value.to_string()).unwrap() },
			_ => { panic!("Unknown char {value}") }
		}
	}
}

#[derive(Debug, Clone)]
pub struct Lawn {
	items: Vec<Vec<MapItem>>,
	zombies: Vec<Vec<usize>>,
	size_x: usize,
	size_y: usize,
}

pub enum ShotResult {
	Miss,
	Hit,
	Kill,
}

impl Lawn {
	pub fn new(lawn: &Vec<&str>, zombies: &Vec<Vec<usize>>) -> Self {
		let size_x = lawn[0].chars().count();
		let size_y = lawn.len();

		let mut items = vec![vec![MapItem::Empty; size_x]; size_y];

		for (y, row) in lawn.iter().enumerate() {
			for (x, char) in row.chars().enumerate() {
				items[y][x] = MapItem::from(char);
			}
		}

		Self {
			items,
			zombies: zombies.clone(),
			size_x: lawn[0].chars().count(),
			size_y: lawn.len(),
		}
	}
	pub fn spawn_zombies(&mut self, turn: usize) {
		for zombie in self.zombies.iter() {
			// Spawn zombies when their turn counter is reached
			let init_turn = zombie[0];
			if init_turn == turn {
				let row = zombie[1];
				let hp = zombie[2];
				self.items[row][self.size_x - 1] = MapItem::Zombie { hp };
			}
		}
		// Remove spawned zombies from list
		self.zombies = self.zombies.clone().into_iter().filter(|z| z[0] != turn).collect();
	}
	pub fn move_zombies(&mut self) {
		let mut move_list = vec![];
		for (y, row) in self.items.iter().enumerate() {
			for (x, item) in row.iter().enumerate() {
				if item.is_zombie() {
					move_list.push((x, y, *item));
				}
			}
		}
		for (x, y, cell) in move_list {
			self.items[y][x] = MapItem::Empty;
			self.items[y][x - 1] = cell;
		}
	}
	pub fn all_zombies_killed(&self) -> bool {
		self.zombies.iter().count() == 0 && self.items.iter().map(|row| row.iter().filter(|e| e.is_zombie()).count()).sum::<usize>() == 0
	}
	pub fn zombie_reached_end(&self) -> bool {
		for row in &self.items {
			for col in row.iter().enumerate() {
				if col.0 == 0 && col.1.is_zombie() {
					return true;
				}
			}
		}
		false
	}
	pub fn shoot_at(&mut self, x: usize, y: usize) -> ShotResult {
		let mut kill = false;
		let mut result = ShotResult::Miss;
		match self.items.get_mut(y).unwrap().get_mut(x).unwrap() {
			MapItem::Zombie { hp } => {
				*hp = hp.saturating_sub(1);
				result = ShotResult::Hit;
				kill = *hp == 0;
			}
			MapItem::SimpleShooter { .. } => {}
			MapItem::SShooter => {}
			MapItem::Empty => {}
		}
		if kill {
			self.items[y][x] = MapItem::Empty;
			result = ShotResult::Kill;
		}
		result
	}
	pub fn shoot_stuff(&mut self) {
		// Simple shooters first
		let mut simple_shooters = vec![];
		for (y, row) in self.items.iter().enumerate() {
			for (x, cell) in row.iter().enumerate() {
				match cell {
					MapItem::SimpleShooter { count } => {
						simple_shooters.push((x, y, *count))
					}
					_ => {}
				}
			}
		}
		// Execute simple shooters shots
		for (x, y, count) in simple_shooters {
			'all_shots: for _ in 0..count {
				let mut offset = 1;
				// Execute shot, looping as long as the bullet misses and continues traveling till the end
				loop {
					if offset + x >= self.size_x {
						break 'all_shots;
					}
					match self.shoot_at(x + offset, y) {
						ShotResult::Miss => {
							offset += 1;
						}
						_ => continue 'all_shots,
					}
				}
			}
		}

		// S shooters next
		let mut s_shooters = vec![];
		// Top to bottom order
		for (y, row) in self.items.iter().enumerate() {
			// Right to left order
			for (x, cell) in row.iter().enumerate().rev() {
				match cell {
					MapItem::SShooter {} => {
						s_shooters.push((x, y))
					}
					_ => {}
				}
			}
		}

		// Execute s shooters shots
		for (x, y) in s_shooters {
			// Execute linear shot
			{
				let mut offset = 1;
				// Execute shot, looping as long as the bullet misses and continues traveling till the end
				loop {
					if offset + x >= self.size_x {
						break;
					}
					match self.shoot_at(x + offset, y) {
						ShotResult::Miss => {
							offset += 1;
						}
						_ => break,
					}
				}
			}
			// Execute upper diagonal
			{
				let mut offset: (i32, i32) = (1, 1);
				// Execute shot, looping as long as the bullet misses and continues traveling till the end
				loop {
					let target_x = x as i32 + offset.0;
					let target_y = y as i32 - offset.1;
					if target_x >= self.size_x as i32 || target_y < 0 {
						break;
					}
					//println!("ud {target_x} {target_y} {} {}", self.size_x, self.size_y);
					match self.shoot_at(target_x as usize, target_y as usize) {
						ShotResult::Miss => {
							offset.0 += 1;
							offset.1 += 1;
						}
						_ => break,
					}
				}
			}
			// Execute lower diagonal
			{
				let mut offset = (1, 1);
				// Execute shot, looping as long as the bullet misses and continues traveling till the end
				loop {
					if x + offset.0 >= self.size_x || y + offset.1 >= self.size_y {
						break;
					}
					match self.shoot_at(x + offset.0, y + offset.1) {
						ShotResult::Miss => {
							offset.0 += 1;
							offset.1 += 1;
						}
						_ => break,
					}
				}
			}
		}
	}
	pub fn index(&self, x: usize, y: usize) -> &MapItem {
		&self.items[y][x]
	}
	pub fn debug(&self) {
		for row in &self.items {
			for cell in row {
				print!("{} ", cell);
			}
			println!();
		}
		println!();
	}
}

mod pnz {
	use super::Lawn;

	pub fn plants_and_zombies(lawn: &Vec<&str>, zombies: &Vec<Vec<usize>>) -> usize {
		let mut lawn = Lawn::new(lawn, zombies);
		for turn in 0.. {
			// lawn.debug();
			if lawn.zombie_reached_end() {
				return turn;
			}
			if lawn.all_zombies_killed() {
				return 0;
			}
			lawn.move_zombies();
			lawn.spawn_zombies(turn);
			lawn.shoot_stuff();
		}

		panic!("Loop terminated before finishing game")
	}
}

#[cfg(test)]
mod example_tests {
	use super::*;

	#[test]
	fn example_tests() {
		let example_tests: Vec<(Vec<&str>, Vec<Vec<usize>>, usize)> = vec![
			(
				vec![
					"2       ",
					"  S     ",
					"21  S   ",
					"13      ",
					"2 3     "],
				vec![
					vec![0, 4, 28],
					vec![1, 1, 6],
					vec![2, 0, 10],
					vec![2, 4, 15],
					vec![3, 2, 16],
					vec![3, 3, 13]],
				10
			),
			(
				vec![
					"11      ",
					" 2S     ",
					"11S     ",
					"3       ",
					"13      "],
				vec![
					vec![0, 3, 16],
					vec![2, 2, 15],
					vec![2, 1, 16],
					vec![4, 4, 30],
					vec![4, 2, 12],
					vec![5, 0, 14],
					vec![7, 3, 16],
					vec![7, 0, 13]],
				12
			),
			(
				vec![
					"12        ",
					"3S        ",
					"2S        ",
					"1S        ",
					"2         ",
					"3         "],
				vec![
					vec![0, 0, 18],
					vec![2, 3, 12],
					vec![2, 5, 25],
					vec![4, 2, 21],
					vec![6, 1, 35],
					vec![6, 4, 9],
					vec![8, 0, 22],
					vec![8, 1, 8],
					vec![8, 2, 17],
					vec![10, 3, 18],
					vec![11, 0, 15],
					vec![12, 4, 21]],
				20
			),
			(
				vec![
					"12      ",
					"2S      ",
					"1S      ",
					"2S      ",
					"3       "],
				vec![
					vec![0, 0, 15],
					vec![1, 1, 18],
					vec![2, 2, 14],
					vec![3, 3, 15],
					vec![4, 4, 13],
					vec![5, 0, 12],
					vec![6, 1, 19],
					vec![7, 2, 11],
					vec![8, 3, 17],
					vec![9, 4, 18],
					vec![10, 0, 15],
					vec![11, 4, 14]],
				19
			),
			(
				vec![
					"1         ",
					"SS        ",
					"SSS       ",
					"SSS       ",
					"SS        ",
					"1         "],
				vec![
					vec![0, 2, 16],
					vec![1, 3, 19],
					vec![2, 0, 18],
					vec![4, 2, 21],
					vec![6, 3, 20],
					vec![7, 5, 17],
					vec![8, 1, 21],
					vec![8, 2, 11],
					vec![9, 0, 10],
					vec![11, 4, 23],
					vec![12, 1, 15],
					vec![13, 3, 22]],
				0
			),
		];

		example_tests.into_iter().for_each(|(grid, zqueue, sol)| assert_eq!(pnz::plants_and_zombies(&grid, &zqueue), sol));
	}
}