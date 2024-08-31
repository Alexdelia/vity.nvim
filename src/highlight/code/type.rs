use crate::hsl;

use nvim_oxi::api::{self, opts::SetHighlightOpts};

pub fn load() -> Result<(), api::Error> {
	let b: bool = true;
	let i: i32 = -42;
	let u: u32 = 42;
	let f: f64 = 42.0;
	let x: usize = 0x42;
	let s: &str = "str";
	let c: char = 'B';

	Ok(())
}
