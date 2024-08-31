use crate::hsl;

use nvim_oxi::api::{self, opts::SetHighlightOpts};

pub fn load() -> Result<(), api::Error> {
	api::set_hl(
		0,
		"Type",
		&SetHighlightOpts::builder()
			.foreground(&hsl(207, 61, 59))
			.italic(false)
			.nocombine(true)
			.build(),
	)?;

	let hue = 140;
	let sat = 73;
	let lum = 59;

	api::set_hl(
		0,
		"Structure",
		&SetHighlightOpts::builder()
			.foreground(&hsl(hue, sat, lum))
			.italic(false)
			.nocombine(true)
			.build(),
	)?;

	api::set_hl(
		0,
		"@type.builtin",
		&SetHighlightOpts::builder()
			.foreground(&hsl(hue + 10, sat + 5, lum - 7))
			.italic(false)
			.nocombine(true)
			.build(),
	)?;
	api::set_hl(
		0,
		"@lsp.type.builtinType",
		&SetHighlightOpts::builder().link("@type.builtin").build(),
	)?;

	api::set_hl(
		0,
		"@lsp.type.typeAlias",
		&SetHighlightOpts::builder()
			.foreground(&hsl(hue + 20, sat, lum))
			.blend(2 / 3)
			.nocombine(true)
			.build(),
	)?;

	Ok(())
}
