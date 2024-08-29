use crate::hsl;

use nvim_oxi::api::{self, opts::SetHighlightOpts};

pub fn load() -> Result<(), api::Error> {
	api::set_hl(
		0,
		"Number",
		&SetHighlightOpts::builder()
			.foreground(&hsl(184, 100, 74))
			.build(),
	)?;

	let bin = SetHighlightOpts::builder()
		.foreground(&hsl(184, 100, 74))
		.build();

	Ok(())
}
