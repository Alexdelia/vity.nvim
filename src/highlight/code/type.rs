use crate::hsl;

use nvim_oxi::api::{self, opts::SetHighlightOpts};

pub fn load() -> Result<(), api::Error> {
	let b: bool = true;
	let i: i32 = -42;
	let u: u32 = 42;
	let f: f64 = 42.0;
	let s: String = "Boolean".to_string();
	let c: char = 'B';

	api::set_hl(
		0,
		"Boolean",
		&SetHighlightOpts::builder()
			.foreground(&hsl(hue, 15, 68))
			.italic(true)
			.build(),
	)?;

	Ok(())
}
