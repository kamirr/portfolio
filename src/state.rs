use serde::{Deserialize, Serialize};
use crate::paths::Paths;

#[derive(Debug, Serialize, Deserialize)]
pub struct Position {
	ticker: String,
	count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Portfolio {
	positions: Vec<Position>
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
		let mut file = std::fs::File::open(path)?;
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

#[derive(Debug)]
pub struct State {
	paths: Paths,
	portfolio: Portfolio
}

impl State {
	pub fn get() -> crate::Result<State> {
		let paths = Paths::get()?;

		let portfolio_path = paths.data.join("portfolio.json");
		let portfolio = Portfolio::from_path(&portfolio_path)?;

		Ok(State {
			paths,
			portfolio,
		})
	}

	pub fn save(&self) -> crate::Result<()> {
		let portfolio_path = self.paths.data.join("portfolio.json");
		self.portfolio.to_path(&portfolio_path)?;

		Ok(())
	}
}
