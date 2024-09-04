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
	let follow = SetHighlightOpts::builder().link("@variable").build();
	api::set_hl(0, "@lsp.type.variable", &follow)?;
	api::set_hl(0, "@lsp.type.property", &follow)?;

	api::set_hl(
		0,
		"@constant",
		&SetHighlightOpts::builder()
			.foreground(&hsl(230, 60, 55))
			.italic(true)
			.nocombine(true)
			.build(),
	)?;
	let follow = SetHighlightOpts::builder().link("@constant").build();
	api::sel_hl(0, "@lsp.type.const", &follow)?;
	api::set_hl(0, "@lsp.mod.constant", &follow)?;
	api::set_hl(0, "@lsp.typemod.const", &follow)?;

	Ok(())
}
