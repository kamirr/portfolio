use serde::{Deserialize, Serialize};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Position {
	pub ticker: String,
	pub count: i32,
}

impl Position {
	pub fn new<T: Into<String>>(ticker: T, count: i32) -> Self {
		Position { ticker: ticker.into(), count }
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Portfolio {
	pub positions: Vec<Position>
}

impl Portfolio {
	pub fn empty() -> Self {
		Portfolio {
			positions: vec![],
		}
	}

	pub fn to_file(&self, file: &mut std::fs::File) -> crate::Result<()> {
		use std::io::prelude::*;

		let serialized = serde_json::to_string(self)?;
		file.write_all(serialized.as_bytes())?;

		Ok(())
	}

	pub fn to_path(&self, path: &std::path::Path) -> crate::Result<()> {
		let mut file = std::fs::File::create(path)?;
		self.to_file(&mut file)?;

		Ok(())
	}

	pub fn from_path(path: &std::path::Path) -> crate::Result<Self> {
		Ok(if path.exists() {
			/* read the portfolio from file */
			let text = std::fs::read_to_string(&path)?;
			serde_json::from_str(&text)?
		} else {
			/* create a file and write an empty portfolio into it */
			let mut file = std::fs::File::create(&path)?;
			Portfolio::empty().to_file(&mut file)?;

			Portfolio::empty()
		})
	}
}
