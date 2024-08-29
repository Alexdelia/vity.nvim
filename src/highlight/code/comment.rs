use crate::hsl;

use nvim_oxi::api::{self, opts::SetHighlightOpts};

pub fn load() -> Result<(), api::Error> {
	let hue = 210;

	api::set_hl(
		0,
		"Comment",
		&SetHighlightOpts::builder()
			.foreground(&hsl(hue, 15, 68))
			.italic(true)
			.build(),
	)?;

	api::set_hl(
		0,
		"@comment.documentation",
		&SetHighlightOpts::builder()
			.foreground(&hsl(hue, 22, 75))
			.italic(true)
			.build(),
	)?;

	api::set_hl(
		0,
		"Todo",
		&SetHighlightOpts::builder()
			.foreground(&hsl(42, 100, 30))
			.italic(false)
			.bold(true)
			.build(),
	)?;

	Ok(())
}
