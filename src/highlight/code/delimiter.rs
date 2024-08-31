use crate::hsl;

use nvim_oxi::api::{self, opts::SetHighlightOpts};

pub fn load() -> Result<(), api::Error> {
	api::set_hl(
		0,
		"Delimiter",
		&SetHighlightOpts::builder()
			.foreground(&hsl(0, 0, 75))
			.build(),
	)?;

	Ok(())
}
