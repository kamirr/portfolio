pub mod aggregator;
pub mod cache;
pub mod iex;
pub mod world_trading_data;

use crate::portfolio::Position;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Response {
	pub price: Option<f32>,
	pub change_pct: Option<f32>,
	pub dividend_yield: Option<f32>,
}

#[derive(Debug, Clone)]
pub struct BulkResponse(HashMap<String, Response>);

impl Default for Response {
	fn default() -> Self {
		Response {
			price: None,
			change_pct: None,
			dividend_yield: None
		}
	}
}

pub trait Backend {
	fn request(&mut self, positions: &[Position]) -> BulkResponse;
}
