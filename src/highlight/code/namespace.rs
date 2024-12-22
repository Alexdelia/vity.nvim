use crate::hsl;

use nvim_oxi::api::{self, opts::SetHighlightOpts};

pub fn load() -> Result<(), api::Error> {
	api::set_hl(
		0,
		"@module",
		&SetHighlightOpts::builder()
			.foreground(&hsl(60, 40, 40))
			.build(),
	)?;

	api::set_hl(
		0,
		"@lsp.type.namespace",
		&SetHighlightOpts::builder().link("@module").build(),
	)?;

	Ok(())
}
