use crate::hsl;

use nvim_oxi::api::{self, opts::SetHighlightOpts};

pub fn load() -> Result<(), api::Error> {
	let hue = 184;
	let sat = 100;
	let lum = 74;

	api::set_hl(
		0,
		"Number",
		&SetHighlightOpts::builder()
			.foreground(&hsl(hue, sat, lum))
			.build(),
	)?;

	let bin = SetHighlightOpts::builder()
		.foreground(&hsl(hue, sat, lum + 6))
		.build();
	api::set_hl(0, "RustBinNumber", &bin)?;

	let oct = SetHighlightOpts::builder()
		.foreground(&hsl(hue, sat, lum - 6))
		.build();
	api::set_hl(0, "RustOctNumber", &oct)?;

	let hex = SetHighlightOpts::builder()
		.foreground(&hsl(hue + 30, sat, lum + 7))
		.build();
	api::set_hl(0, "RustHexNumber", &hex)?;
	api::set_hl(0, "DevIconHexadecimal", &hex)?;

	let float = SetHighlightOpts::builder()
		.foreground(&hsl(hue - 10, sat, lum))
		.build();
	api::set_hl(0, "Float", &float)?;
	api::set_hl(0, "NvimFloat", &float)?;

	Ok(())
}
