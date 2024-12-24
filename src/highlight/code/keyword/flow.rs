use crate::hsl;

use nvim_oxi::api::{self, opts::SetHighlightOpts};

pub fn load() -> Result<(), api::Error> {
	api::set_hl(
		0,
		"Conditional",
		&SetHighlightOpts::builder()
			.foreground(&hsl(45, 81, 55))
			.build(),
	)?;

	api::set_hl(
		0,
		"Repeat",
		&SetHighlightOpts::builder()
			.foreground(&hsl(293, 61, 62))
			.build(),
	)?;

	Ok(())
}
