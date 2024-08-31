use crate::hsl;

use nvim_oxi::api::{self, opts::SetHighlightOpts};

pub fn load() -> Result<(), api::Error> {
	api::set_hl(
		0,
		"Identifier",
		&SetHighlightOpts::builder()
			.foreground(&hsl(60, 53, 55))
			.blend(2 / 3)
			.italic(false)
			.nocombine(true)
			.build(),
	)?;

	Ok(())
}
