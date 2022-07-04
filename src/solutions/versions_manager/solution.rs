// mod preloaded;// do not modify this!
// use preloaded::VMError;


#[derive(Eq, PartialEq, Debug)]
pub enum VMError {
	InvalidVersion, // for the `from_version` function
	NoHistory,      // for the `rollback` method
}

use std::str::FromStr;

#[derive(Clone, Eq, PartialEq, Debug)]
struct VersionManager {
	major: u16,
	minor: u16,
	patch: u16,
	backup: Option<Box<Self>>,
}

impl Default for VersionManager {
	fn default() -> Self {
		VersionManager {
			major: 0,
			minor: 0,
			patch: 1,
			backup: None
		}
	}
}

impl VersionManager {

	fn new() -> Self {
		Self::default()
	}

	fn from_version(version: &str) -> Result<Self, VMError> {
		if version == "" {
			return Ok(Self::default())
		}

		let split = version.split('.').collect::<Vec<&str>>();

		for i in 0..3 {
			if let Some(item) = split.get(i) {
				if u16::from_str(item).is_err() {
					return Err(VMError::InvalidVersion);
				}
			}
		}

		let from_idx_to_ver = |index| u16::from_str(split[index]).map_err(|_|VMError::InvalidVersion);

		let major = from_idx_to_ver(0)?;

		let minor = if split.get(1).is_some() {
			from_idx_to_ver(1)?
		} else {
			0
		};

		let patch = if split.get(2).is_some() {
			from_idx_to_ver(2)?
		} else {
			0
		};

		Ok(Self {
			major,
			minor,
			patch,
			backup: None
		})
	}

	fn major(&mut self) -> &mut Self {
		self.store_previous();
		self.major += 1;
		self.minor = 0;
		self.patch = 0;
		self
	}

	fn minor(&mut self) -> &mut Self {
		self.store_previous();
		self.minor += 1;
		self.patch = 0;
		self
	}

	fn patch(&mut self) -> &mut Self {
		self.store_previous();
		self.patch += 1;
		self
	}

	fn store_previous(&mut self) {
		self.backup = Some(Box::new(self.clone()));
	}

	fn rollback(&mut self) -> Result<&mut Self, VMError> {
		let backup = self.backup.clone();
		return if let Some(backup) = backup {
			self.backup = None;
			*self = *backup;
			Ok(self)
		} else {
			Err(VMError::NoHistory)
		}
	}

	fn release(&self) -> String {
		format!("{}.{}.{}", self.major, self.minor, self.patch)
	}
}


use crate::solutions::versions_manager::solution::VMError::InvalidVersion;


#[test]
fn from_invalid_version() {

	assert_eq!(VersionManager::from_version("1.2.c"), Err(InvalidVersion));
}