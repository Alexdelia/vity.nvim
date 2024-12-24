use crate::hsl;

use nvim_oxi::api::{self, opts::SetHighlightOpts};

pub fn load() -> Result<(), api::Error> {
	api::set_hl(
		0,
		"@keyword.function",
		&SetHighlightOpts::builder()
			.foreground(&hsl(207, 35, 35))
			.build(),
	)?;

	Ok(())
}
