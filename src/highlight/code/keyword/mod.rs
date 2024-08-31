mod flow;

use nvim_oxi::api::{self, opts::SetHighlightOpts};

use crate::hsl;

pub fn load() -> Result<(), api::Error> {
	api::set_hl(
		0,
		"Keyword",
		&SetHighlightOpts::builder()
			.foreground(&hsl(54, 66, 46))
			.build(),
	)?;

	flow::load()?;

	Ok(())
}
