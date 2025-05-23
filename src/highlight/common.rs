use nvim_oxi::api::{self, opts::SetHighlightOpts};

use crate::hsl::convert::to_rgb::hsl;

pub fn load() -> Result<(), api::Error> {
	api::set_hl(
		0,
		"Directory",
		&SetHighlightOpts::builder()
			.foreground(&hsl(200, 80, 50))
			.build(),
	)?;

	api::set_hl(
		0,
		"NonText",
		&SetHighlightOpts::builder().foreground("NONE").build(),
	)?;

	Ok(())
}
