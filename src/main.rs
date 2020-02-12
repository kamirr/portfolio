mod state;
mod paths;
mod error;

pub type Result<T> = std::result::Result<T, error::Error>;

fn main() {
	let s = state::State::get();
	println!("state: {:#?}", s);
	std::io::stdin().read_line(&mut String::new()).unwrap();
	s.unwrap().save();
}
