use crate::backend::{Backend, BulkResponse, Response};
use crate::portfolio::Position;
use serde_json::Value;
use std::collections::HashMap;

pub struct WorldTradingData {
	api_key: String
}

impl WorldTradingData {
	pub fn new(api_key: String) -> Box<dyn Backend> {
		Box::new(WorldTradingData { api_key })
	}

	fn make_request(&self, positions: &[Position]) -> serde_json::Value {
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

impl Backend for WorldTradingData {
	fn request(&self, positions: &[Position]) -> BulkResponse {
		let mut result = HashMap::new();

		for ticker in positions.iter().map(|p| &p.ticker) {
			result.insert(ticker.clone(), Response::default());
		}

		let response = self.make_request(positions);
		for item in response_to_data_iter(response) {
			if let Some(symbol) = item["symbol"].as_str() {
				result
					.get_mut(symbol.into())
					.map(|p| p.price = get_entry(&item, "price"));
				result
					.get_mut(symbol.into())
					.map(|p| p.change_pct = get_entry(&item, "day_change"));
			}
		}

		BulkResponse(result)
	}
}

fn response_to_data_iter(response: Value) -> Vec<Value> {
	response
		.get("data")
		.and_then(Value::as_array)
		.map_or(Vec::new(), Vec::clone)
}

fn get_entry(item: &Value, param: &str) -> Option<f32> {
	item[param]
		.as_str()
		.and_then(|s| s.parse::<f32>().ok())
}
