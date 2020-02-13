#[allow(dead_code)]

mod portfolio;
mod backend;
mod state;
mod paths;
mod error;

pub type Result<T> = std::result::Result<T, error::Error>;

fn main() {
	use backend::{
		world_trading_data::WorldTradingData,
		aggregator::Aggregator,
		cache::Cache,
	};

	let state = state::State::get().unwrap();
	println!("state: {:#?}", state);
	state.save().unwrap();

	let backend = WorldTradingData::new("demo".to_owned());
	let cached_backend = Cache::new(backend, std::time::Duration::from_secs(1));
	let mut aggregator = Aggregator::new(vec![cached_backend]);
	println!("{:#?}", aggregator.request(&state));
	println!("{:#?}", aggregator.request(&state));
	std::thread::sleep(std::time::Duration::from_secs(2));
	println!("{:#?}", aggregator.request(&state));
}
