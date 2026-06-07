use crate::hsl;

use nvim_oxi::api::{self, opts::SetHighlightOpts};

pub fn load() -> Result<(), api::Error> {
	api::set_hl(
		0,
		"Delimiter",
		&SetHighlightOpts::builder()
			.foreground(&hsl(0, 0, 66))
			.build(),
	)?;
	let f = follow!("Delimiter");
	api::set_hl(0, "@tag.delimiter", f)?;

	api::set_hl(
		0,
		"@constructor.lua",
		&SetHighlightOpts::builder()
			.foreground(&hsl(0, 0, 66))
			.nocombine(true)
			.build(),
	)?;

	api::set_hl(
		0,
		"@markup.raw.block.markdown",
		&SetHighlightOpts::builder()
			.foreground(&hsl(40, 90, 45))
			.build(),
	)?;

	Ok(())
}
