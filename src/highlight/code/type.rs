use crate::{hsl, Hsl};

use nvim_oxi::api::{self, opts::SetHighlightOpts};

pub fn load() -> Result<(), api::Error> {
	api::set_hl(
		0,
		"Type",
		&SetHighlightOpts::builder()
			.foreground(&hsl(207, 61, 59))
			.italic(false)
			.nocombine(true)
			.build(),
	)?;

	let base = Hsl {
		h: 140,
		s: 73,
		l: 59,
	};

	api::set_hl(
		0,
		"Structure",
		&SetHighlightOpts::builder()
			.foreground(&base.to_rgb())
			.italic(false)
			.nocombine(true)
			.build(),
	)?;

	api::set_hl(
		0,
		"@type.builtin",
		&SetHighlightOpts::builder()
			.foreground(&hsl(base.h + 10, base.s + 5, base.l - 7))
			.italic(false)
			.nocombine(true)
			.build(),
	)?;
	api::set_hl(
		0,
		"@lsp.type.builtinType",
		&SetHighlightOpts::builder().link("@type.builtin").build(),
	)?;

	api::set_hl(
		0,
		"@lsp.type.typeAlias",
		&SetHighlightOpts::builder()
			.foreground(&hsl(base.h + 20, base.s, base.l))
			.blend(2 / 3)
			.nocombine(true)
			.build(),
	)?;

	Ok(())
}
