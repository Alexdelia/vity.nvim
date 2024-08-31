use nvim_oxi::api::{self, opts::SetHighlightOpts};

use crate::hsl;

pub fn load() -> Result<(), api::Error> {
	// TODO
	api::set_hl(
		0,
		"BufferLineFill",
		&SetHighlightOpts::builder()
			.foreground(&hsl(0, 0, 50))
			.build(),
	)?;

	Ok(())
}
