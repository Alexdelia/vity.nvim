mod declaration;
mod flow;
mod import;
mod modifier;

use nvim_oxi::api::{self, opts::SetHighlightOpts};

use crate::hsl;

pub fn load() -> Result<(), api::Error> {
	api::set_hl(
		0,
		"Keyword",
		&SetHighlightOpts::builder()
			.foreground(&hsl(207, 61, 59))
			.build(),
	)?;

	flow::load()?;
	declaration::load()?;
	import::load()?;
	modifier::load()?;

	Ok(())
}
