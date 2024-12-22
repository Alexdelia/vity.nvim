use nvim_oxi::api::{self, opts::SetHighlightOpts};

use const_format::formatcp as f;

use crate::hsl::hsl;

const PRE: &str = "Telescope";

pub fn load() -> Result<(), api::Error> {
	api::set_hl(
		0,
		f!("{PRE}Matching"),
		&SetHighlightOpts::builder()
			.foreground(&hsl(0, 66, 50))
			.bold(true)
			.build(),
	)?;

	Ok(())
}
