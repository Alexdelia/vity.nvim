use crate::{hsl, Hsl};

use nvim_oxi::api::{self, opts::SetHighlightOpts};

pub fn load() -> Result<(), api::Error> {
	let var = Hsl {
		h: 180,
		s: 74,
		l: 95,
	};

	api::set_hl(
		0,
		"@variable",
		&SetHighlightOpts::builder()
			.foreground(&var.to_rgb())
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
	api::set_hl(0, "@lsp.type.const", f)?;

	api::set_hl(
		0,
		"@variable.builtin",
		&SetHighlightOpts::builder()
			.foreground(
				&Hsl {
					h: 280,
					s: var.s,
					l: var.l - 10,
				}
				.to_rgb(),
			)
			.italic(true)
			.nocombine(true)
			.build(),
	)?;

	Ok(())
}
