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
	api::set_hl(0, "@constructor.lua", f)?;

	Ok(())
}
