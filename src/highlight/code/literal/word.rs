use crate::hsl;

use nvim_oxi::api::{self, opts::SetHighlightOpts};

pub fn load() -> Result<(), api::Error> {
	api::set_hl(
		0,
		"Boolean",
		&SetHighlightOpts::builder()
			.foreground(&hsl(227, 63, 66))
			.italic(true)
			.build(),
	)?;
	// TODO: null

	Ok(())
}
