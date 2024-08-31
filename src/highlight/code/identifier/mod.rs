mod function;
mod variable;

use crate::hsl;

use nvim_oxi::api::{self, opts::SetHighlightOpts};

pub fn load() -> Result<(), api::Error> {
	api::set_hl(
		0,
		"Identifier",
		&SetHighlightOpts::builder()
			.foreground(&hsl(180, 74, 95))
			.italic(true)
			.build(),
	)?;

	variable::load()?;
	function::load()?;

	Ok(())
}
