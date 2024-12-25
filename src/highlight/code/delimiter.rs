use crate::hsl;

use nvim_oxi::api::{self, opts::SetHighlightOpts};

pub fn load() -> Result<(), api::Error> {
	api::set_hl(
		0,
		"Delimiter",
		&SetHighlightOpts::builder()
			.foreground(&hsl(0, 0, 66))
			.build(),
	)?;

	api::set_hl(
		0,
		"@constructor.lua",
		&SetHighlightOpts::builder()
			.foreground(&hsl(0, 0, 66))
			.nocombine(true)
			.build(),
	)?;

	Ok(())
}
