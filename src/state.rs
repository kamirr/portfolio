use crate::paths::Paths;
use crate::portfolio::Portfolio;

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
