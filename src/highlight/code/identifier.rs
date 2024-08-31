use crate::hsl;

use nvim_oxi::api::{self, opts::SetHighlightOpts};

pub fn load() -> Result<(), api::Error> {
	api::set_hl(
		0,
		"Identifier",
		&SetHighlightOpts::builder()
			.foreground(&hsl(180, 74, 95))
			.italic(true)
			.build(),
	)?;
	api::set_hl(
		0,
		"@variable",
		&SetHighlightOpts::builder()
			.foreground(&hsl(180, 74, 95))
			.italic(true)
			.build(),
	)?;
	api::set_hl(
		0,
		"@lsp.type.variable",
		&SetHighlightOpts::builder().link("@variable").build(),
	)?;

	function()?;

	Ok(())
}

fn function() -> Result<(), api::Error> {
	let hue = 336;
	let sat = 73;
	let lum = 59;

	api::set_hl(
		0,
		"Function",
		&SetHighlightOpts::builder()
			.foreground(&hsl(hue, sat, lum))
			.italic(false)
			.bold(true)
			.nocombine(true)
			.build(),
	)?;

	api::set_hl(
		0,
		"Macro",
		&SetHighlightOpts::builder()
			.foreground(&hsl(hue - 13, sat, lum))
			.italic(false)
			.bold(true)
			.nocombine(true)
			.build(),
	)?;

	api::set_hl(
		0,
		"@function.builtin",
		&SetHighlightOpts::builder().link("Macro").build(),
	)?;

	Ok(())
}
