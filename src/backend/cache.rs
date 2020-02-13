use crate::backend::{Backend, BulkResponse, Response};
use crate::portfolio::Position;
use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct Cache {
	backend: Box<dyn Backend>,
	max_delay: Duration,
	responses: HashMap<String, (Response, Instant)>
}

impl Cache {
	pub fn new(backend: Box<dyn Backend>, max_delay: Duration) -> Box<dyn Backend> {
		Box::new(Cache {
			backend,
			max_delay,
			responses: HashMap::new()
		})
	}
}

impl Backend for Cache {
	fn request(&mut self, positions: &[Position]) -> BulkResponse {
		let mut result = HashMap::<String, Response>::new();
		let mut uncached_positions = Vec::new();

		for p in positions {
			let found = if let Some((response, recorded_at)) = self.responses.get(&p.ticker) {
				if recorded_at.elapsed() < self.max_delay {
					result.insert(p.ticker.clone(), response.clone());
					true
				} else {
					false
				}
			} else {
				false
			};

			if !found {
				uncached_positions.push(p.clone());
			}
		}

		if !uncached_positions.is_empty() {
			let fresh_result = self.backend.request(&uncached_positions);

			for (k, v) in &fresh_result.0 {
				self.responses.insert(k.clone(), (v.clone(), Instant::now()));
			}

			result.extend(fresh_result.0);
		}

		BulkResponse(result)
	}
}
