use crate::hsl;

use nvim_oxi::api::{self, opts::SetHighlightOpts};

pub fn load() -> Result<(), api::Error> {
	api::set_hl(
		0,
		"Type",
		&SetHighlightOpts::builder()
			.foreground(&hsl(207, 61, 59))
			.italic(false)
			.build(),
	)?;

	api::set_hl(
		0,
		"Structure",
		&SetHighlightOpts::builder()
			.foreground(&hsl(140, 73, 59))
			.italic(false)
			.build(),
	)?;

	api::set_hl(
		0,
		"@type.builtin",
		&SetHighlightOpts::builder()
			.foreground(&hsl(151, 76, 52))
			.italic(false)
			.build(),
	)?;
	api::set_hl(
		0,
		"@lsp.type.builtinType",
		&SetHighlightOpts::builder().link("@type.builtin").build(),
	)?;

	Ok(())
}
