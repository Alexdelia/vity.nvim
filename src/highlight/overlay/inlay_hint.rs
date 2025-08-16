use nvim_oxi::api::{self, opts::SetHighlightOpts};

use crate::hsl::convert::to_rgb::hsl;

pub fn load() -> Result<(), api::Error> {
	api::set_hl(
		0,
		"LspInlayHint",
		&SetHighlightOpts::builder()
			.foreground(&hsl(0, 0, 60))
			.background(&hsl(0, 0, 16))
			.build(),
	)?;

	Ok(())
}
