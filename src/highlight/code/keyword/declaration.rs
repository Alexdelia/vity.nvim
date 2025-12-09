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

	api::set_hl(
		0,
		"@markup.list",
		&SetHighlightOpts::builder()
			.foreground(&hsl(0, 0, 66))
			.build(),
	)?;
	api::set_hl(
		0,
		"@markup.list.unchecked",
		&SetHighlightOpts::builder()
			.foreground(&hsl(205, 100, 66))
			.build(),
	)?;
	api::set_hl(
		0,
		"@markup.list.checked",
		&SetHighlightOpts::builder()
			.foreground(&hsl(120, 66, 66))
			.build(),
	)?;

	Ok(())
}
