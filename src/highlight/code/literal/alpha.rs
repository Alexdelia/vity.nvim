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

	let hue_char = hue - 30;
	api::set_hl(
		0,
		"Char",
		&SetHighlightOpts::builder()
			.foreground(&hsl(hue_char, sat, lum))
			.build(),
	)?;
	api::set_hl(
		0,
		"Character",
		&SetHighlightOpts::builder().link("Char").build(),
	)?;

	let lum_quote = lum - 20;
	api::set_hl(
		0,
		"NvimStringQuote",
		&SetHighlightOpts::builder()
			.foreground(&hsl(hue, sat, lum_quote))
			.build(),
	)?;
	api::set_hl(
		0,
		"NvimDoubleQuote",
		&SetHighlightOpts::builder().link("NvimStringQuote").build(),
	)?;

	api::set_hl(
		0,
		"NvimSingleQuote",
		&SetHighlightOpts::builder()
			.foreground(&hsl(hue_char, sat, lum_quote))
			.build(),
	)?;

	Ok(())
}
