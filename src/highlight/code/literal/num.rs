use crate::hsl;

use nvim_oxi::api::{self, opts::SetHighlightOpts};

pub fn load() -> Result<(), api::Error> {
	let hue = 184;

	api::set_hl(
		0,
		"Number",
		&SetHighlightOpts::builder()
			.foreground(&hsl(hue, 100, 74))
			.build(),
	)?;

	let bin = SetHighlightOpts::builder()
		.foreground(&hsl(hue, 100, 80))
		.build();
	api::set_hl(0, "RustBinNumber", &bin)?;

	let oct = SetHighlightOpts::builder()
		.foreground(&hsl(hue, 100, 68))
		.build();
	api::set_hl(0, "RustOctNumber", &oct)?;

	let hex = SetHighlightOpts::builder()
		.foreground(&hsl(hue + 30, 100, 81))
		.build();
	api::set_hl(0, "RustHexNumber", &hex)?;
	api::set_hl(0, "DevIconHexadecimal", &hex)?;

	Ok(())
}
