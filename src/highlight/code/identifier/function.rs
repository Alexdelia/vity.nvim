use crate::{hsl, Hsl};

use nvim_oxi::api::{self, opts::SetHighlightOpts};

pub fn load() -> Result<(), api::Error> {
	let base = Hsl {
		h: 336,
		s: 73,
		l: 59,
	};

	api::set_hl(
		0,
		"Function",
		&SetHighlightOpts::builder()
			.foreground(&base.to_rgb())
			.italic(false)
			.bold(true)
			.nocombine(true)
			.build(),
	)?;

	api::set_hl(
		0,
		"Macro",
		&SetHighlightOpts::builder()
			.foreground(&hsl(base.h - 13, base.s, base.l))
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
