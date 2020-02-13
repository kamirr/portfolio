use crate::portfolio::Position;

pub struct WorldTradingData {
	api_key: String
}

impl WorldTradingData {
	pub fn new(api_key: String) -> Self {
		WorldTradingData { api_key }
	}

	fn request(&self, positions: &[Position]) -> serde_json::Value {
		let url = self.construct_query(positions);
		let response = reqwest::blocking::get(&url).unwrap().text().unwrap();
		serde_json::from_str(&response).unwrap()
	}

	fn construct_query(&self, positions: &[Position]) -> String {
		assert!(positions.len() > 0 && positions.len() <= 5);

		let symbols = positions
			.iter()
			.map(|p| &p.ticker)
			.fold(String::new(), |acc, r| { format!("{},{}", &r, &acc) });

		format!(
			"https://api.worldtradingdata.com/api/v1/stock?symbol={}&api_token={}",
			&symbols[0 .. symbols.len() - 1],
			&self.api_key
		)
	}
}
