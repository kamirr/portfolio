#[allow(dead_code)]

mod portfolio;
mod backend;
mod state;
mod paths;
mod error;

pub type Result<T> = std::result::Result<T, error::Error>;

fn main() {
	use portfolio::Position;
    use backend::Backend;

	let api_key = "demo".to_owned();
	let b = backend::world_trading_data::WorldTradingData::new(api_key);
	let p = [Position::new("VOD.L", 0), Position::new("TWTR", 2), Position::new("SNAP", 1),];
	println!("{:#?}", b.request(&p));

	let s = state::State::get().unwrap();
	println!("state: {:#?}", s);
	s.save().unwrap();
}
