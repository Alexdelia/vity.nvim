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
	let f = follow!("@variable");
	api::set_hl(0, "@lsp.type.variable", f)?;
	api::set_hl(0, "@lsp.type.property", f)?;

	api::set_hl(
		0,
		"Constant",
		&SetHighlightOpts::builder()
			.foreground(&hsl(230, 60, 55))
			.nocombine(true)
			.build(),
	)?;
	let f = follow!("Constant");
	api::set_hl(0, "@constant", f)?;

	Ok(())
}
