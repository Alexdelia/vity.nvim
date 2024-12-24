use crate::hsl;

use nvim_oxi::api::{self, opts::SetHighlightOpts};

pub fn load() -> Result<(), api::Error> {
	api::set_hl(
		0,
		"@keyword.import",
		&SetHighlightOpts::builder()
			.foreground(&hsl(265, 78, 65))
			.build(),
	)?;

	Ok(())
}
