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

	api::set_hl(
		0,
		"Function",
		&SetHighlightOpts::builder()
			.foreground(&hsl(336, 73, 59))
			.italic(false)
			.bold(true)
			.nocombine(true)
			.build(),
	)?;

	Ok(())
}
