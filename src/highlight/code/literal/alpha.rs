use crate::{hsl, Hsl};

use nvim_oxi::api::{self, opts::SetHighlightOpts};

pub fn load() -> Result<(), api::Error> {
	let base = Hsl {
		h: 95,
		s: 48,
		l: 55,
	};

	api::set_hl(
		0,
		"String",
		&SetHighlightOpts::builder()
			.foreground(&base.to_rgb())
			.build(),
	)?;

	let hue_char = base.h - 30;
	api::set_hl(
		0,
		"Char",
		&SetHighlightOpts::builder()
			.foreground(&hsl(hue_char, base.s, base.l))
			.build(),
	)?;
	api::set_hl(
		0,
		"Character",
		&SetHighlightOpts::builder().link("Char").build(),
	)?;

	let lum_quote = base.l - 20;
	api::set_hl(
		0,
		"NvimStringQuote",
		&SetHighlightOpts::builder()
			.foreground(&hsl(base.h, base.s, lum_quote))
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
			.foreground(&hsl(hue_char, base.s, lum_quote))
			.build(),
	)?;

	Ok(())
}
