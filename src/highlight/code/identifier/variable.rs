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

	let constant = Hsl {
		h: 230,
		s: 60,
		l: 55,
	};
	api::set_hl(
		0,
		"Constant",
		&SetHighlightOpts::builder()
			.foreground(&constant.to_rgb())
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
			.foreground(&hsl(280, var.s, var.l - 10))
			.italic(true)
			.nocombine(true)
			.build(),
	)?;
	api::set_hl(
		0,
		"@constant.builtin",
		&SetHighlightOpts::builder()
			.foreground(&hsl(constant.h + 25, constant.s, constant.l))
			.nocombine(true)
			.build(),
	)?;

	Ok(())
}
