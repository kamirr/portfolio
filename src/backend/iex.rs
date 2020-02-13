use crate::backend::{Backend, BulkResponse, Response};
use crate::portfolio::Position;
use serde_json::Value;
use std::collections::HashMap;

pub struct Caps {
	dividend: bool,
	change: bool,
	price: bool,
}

impl Default for Caps {
	fn default() -> Self {
		Caps {
			dividend: false,
			change: false,
			price: false
		}
	}
}

impl Caps {
	pub fn toggle_dividend(mut self) -> Self {
		self.dividend = !self.dividend;
		self
	}
	pub fn toggle_change(mut self) -> Self {
		self.change = !self.change;
		self
	}
	pub fn toggle_price(mut self) -> Self {
		self.price = !self.price;
		self
	}
}

pub struct Iex {
	api_key: String,
	caps: Caps,
}

impl Iex {
	pub fn new(api_key: String, caps: Caps) -> Box<dyn Backend> {
		Box::new(Iex { api_key, caps })
	}

	fn make_request(&self, ticker: &str) -> Option<serde_json::Value> {
		let url = format!(
			"https://cloud.iexapis.com/stable/stock/{}/dividends/6m?token={}",
			ticker,
			&self.api_key
		);

		let response = reqwest::blocking::get(&url).unwrap().text().unwrap();

		println!("{:?}", &response);
		let result = serde_json::from_str(&response).ok();

		println!("{:?}", &result);
		result
	}

	fn request_one(&self, ticker: &str) -> Response {
		self.make_request(ticker);
		Default::default()
	}
}

impl Backend for Iex {
	fn request(&mut self, positions: &[Position]) -> BulkResponse {
		let mut result = HashMap::new();

		for p in positions {
			result.insert(p.ticker.clone(), self.request_one(&p.ticker));
		}

		BulkResponse(result)
	}
}
