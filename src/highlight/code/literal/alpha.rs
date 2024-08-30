use crate::hsl;

use nvim_oxi::api::{self, opts::SetHighlightOpts};

pub fn load() -> Result<(), api::Error> {
	let hue = 95;
	let sat = 48;
	let lum = 55;

	api::set_hl(
		0,
		"String",
		&SetHighlightOpts::builder()
			.foreground(&hsl(hue, sat, lum))
			.build(),
	)?;

	api::set_hl(
		0,
		"Char",
		&SetHighlightOpts::builder()
			.foreground(&hsl(hue - 30, sat, lum))
			.build(),
	)?;

	// TODO: darker quote
	api::set_hl(
		0,
		"NvimStringQuote",
		&SetHighlightOpts::builder()
			.foreground(&hsl(hue - 30, sat, lum))
			.build(),
	)?;

	Ok(())
}
