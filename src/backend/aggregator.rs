use crate::backend::{Backend, BulkResponse};
use crate::state::State;

pub struct Aggregator {
	backends: Vec<Box<dyn Backend>>
}

impl Aggregator {
	pub fn new(backends: Vec<Box<dyn Backend>>) -> Self {
		Aggregator { backends }
	}

	pub fn request(&self, state: &State) -> Option<BulkResponse> {
		self.backends
			.first()
			.map(|b| b.request(&state.portfolio.positions))
	}
}
