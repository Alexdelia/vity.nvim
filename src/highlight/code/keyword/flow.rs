use crate::{Hsl, hsl};

use nvim_oxi::api::{self, opts::SetHighlightOpts};

pub fn load() -> Result<(), api::Error> {
	let base = Hsl {
		h: 45,
		s: 81,
		l: 55,
	};
	api::set_hl(
		0,
		"Conditional",
		&SetHighlightOpts::builder()
			.foreground(&base.to_rgb())
			.build(),
	)?;
	let f = follow!("Conditional");
	api::set_hl(0, "@keyword.conditional", f)?;
	api::set_hl(0, "@lsp.mod.controlFlow", f)?;
	api::set_hl(0, "@lsp.typemod.operator.controlFlow", f)?;
	api::set_hl(0, "@lsp.typemod.keyword.control", f)?;
	api::set_hl(0, "rustQuestionMark", f)?;

	api::set_hl(
		0,
		"@keyword.return",
		&SetHighlightOpts::builder()
			.foreground(&hsl(base.h, base.s - 5, base.l - 5))
			.build(),
	)?;

	api::set_hl(
		0,
		"@keyword.operator",
		&SetHighlightOpts::builder()
			.foreground(&hsl(base.h + 17, base.s, base.l))
			.build(),
	)?;

	api::set_hl(
		0,
		"Repeat",
		&SetHighlightOpts::builder()
			.foreground(&hsl(293, 61, 62))
			.build(),
	)?;
	let f = follow!("Repeat");
	api::set_hl(0, "@keyword.repeat", f)?;

	let f = follow!("Normal");
	api::set_hl(0, "dartStatement", f)?;
	api::set_hl(0, "dartLabel", f)?;
	api::set_hl(0, "dartExceptions", f)?;

	Ok(())
}
