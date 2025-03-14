use crate::hsl;

use nvim_oxi::api::{self, opts::SetHighlightOpts};

pub fn load() -> Result<(), api::Error> {
	api::set_hl(
		0,
		"@tag",
		&SetHighlightOpts::builder()
			.foreground(&hsl(200, 60, 50))
			.bold(true)
			.build(),
	)?;

	Ok(())
}
