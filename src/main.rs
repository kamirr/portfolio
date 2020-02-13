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
	};

	let state = state::State::get().unwrap();
	println!("state: {:#?}", state);
	state.save().unwrap();

	let backend = WorldTradingData::new("demo".to_owned());
	let aggregator = Aggregator::new(vec![backend]);
	println!("{:#?}", aggregator.request(&state));
}
