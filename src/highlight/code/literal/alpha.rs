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
		"Character",
		&SetHighlightOpts::builder()
			.foreground(&hsl(hue_char, base.s, base.l))
			.build(),
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

	let interpolation = SetHighlightOpts::builder()
		.foreground(&hsl(280, 63, 47))
		.build();
	api::set_hl(0, "@lsp.type.formatSpecifier", &interpolation)?;
	api::set_hl(0, "@punctuation.special.bash", &interpolation)?;

	let path = SetHighlightOpts::builder()
		.foreground(&hsl(80, 50, 50))
		.build();
	api::set_hl(0, "@string.special.path", &path)?;
	api::set_hl(0, "@lsp.type.path", &path)?;

	Ok(())
}
