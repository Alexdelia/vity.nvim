use crate::{hsl, Hsl};

use nvim_oxi::api::{self, opts::SetHighlightOpts};

pub fn load() -> Result<(), api::Error> {
	let base = Hsl {
		h: 184,
		s: 100,
		l: 74,
	};

	api::set_hl(
		0,
		"Number",
		&SetHighlightOpts::builder()
			.foreground(&base.to_rgb())
			.build(),
	)?;

	let bin = SetHighlightOpts::builder()
		.foreground(&hsl(base.h, base.s, base.l + 6))
		.build();
	api::set_hl(0, "RustBinNumber", &bin)?;

	let oct = SetHighlightOpts::builder()
		.foreground(&hsl(base.h, base.s, base.l - 6))
		.build();
	api::set_hl(0, "RustOctNumber", &oct)?;

	let hex = SetHighlightOpts::builder()
		.foreground(&hsl(base.h + 30, base.s, base.l + 7))
		.build();
	api::set_hl(0, "RustHexNumber", &hex)?;
	api::set_hl(0, "DevIconHexadecimal", &hex)?;

	api::set_hl(
		0,
		"Float",
		&SetHighlightOpts::builder()
			.foreground(&hsl(base.h - 10, base.s, base.l))
			.build(),
	)?;
	api::set_hl(
		0,
		"NvimFloat",
		&SetHighlightOpts::builder().link("Float").build(),
	)?;
	api::set_hl(
		0,
		"@number.float",
		&SetHighlightOpts::builder().link("Float").build(),
	)?;

	Ok(())
}
