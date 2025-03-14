use crate::hsl;

use nvim_oxi::api::{self, opts::SetHighlightOpts};

pub fn load() -> Result<(), api::Error> {
	api::set_hl(
		0,
		"@tag",
		&SetHighlightOpts::builder()
			.foreground(&hsl(200, 80, 50))
			.build(),
	)?;

	Ok(())
}
