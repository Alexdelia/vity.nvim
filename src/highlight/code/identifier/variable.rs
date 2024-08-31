use crate::hsl;

use nvim_oxi::api::{self, opts::SetHighlightOpts};

pub fn load() -> Result<(), api::Error> {
	api::set_hl(
		0,
		"@variable",
		&SetHighlightOpts::builder()
			.foreground(&hsl(180, 74, 95))
			.italic(true)
			.nocombine(true)
			.build(),
	)?;
	api::set_hl(
		0,
		"@lsp.type.variable",
		&SetHighlightOpts::builder().link("@variable").build(),
	)?;
	api::set_hl(
		0,
		"@lsp.type.property",
		&SetHighlightOpts::builder().link("@variable").build(),
	)?;

	Ok(())
}
