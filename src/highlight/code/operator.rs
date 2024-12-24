use crate::hsl;

use nvim_oxi::api::{self, opts::SetHighlightOpts};

pub fn load() -> Result<(), api::Error> {
	// operator should have multiple highlight group
	// - assignment (green)
	// - arithmetic (red/pink)
	// - access (cyan)
	// - conditional (yellow/orange)
	// but for now, I need to compromise
	api::set_hl(
		0,
		"Operator",
		&SetHighlightOpts::builder()
			.foreground(&hsl(280, 100, 90))
			.build(),
	)?;

	Ok(())
}
