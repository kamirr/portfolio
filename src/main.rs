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
		iex::{Iex, Caps},
	};

	let state = state::State::get().unwrap();
	println!("state: {:#?}", state);
	state.save().unwrap();

	let _cached_wtd = Cache::new(
		WorldTradingData::new("demo".to_owned()),
		std::time::Duration::from_secs(1)
	);

	let cached_iex = Cache::new(
		Iex::new("pk_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx".to_owned(), Caps::default().toggle_dividend()),
		std::time::Duration::from_secs(3600 * 24)
	);

	let mut aggregator = Aggregator::new(vec![cached_iex]);
	println!("{:#?}", aggregator.request(&state));
	println!("{:#?}", aggregator.request(&state));
	std::thread::sleep(std::time::Duration::from_secs(2));
	println!("{:#?}", aggregator.request(&state));
}
