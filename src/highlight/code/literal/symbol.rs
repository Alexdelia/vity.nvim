use crate::hsl;

use nvim_oxi::api::{self, opts::SetHighlightOpts};

pub fn load() -> Result<(), api::Error> {
	api::set_hl(
		0,
		"@punctuation.special.rust",
		&SetHighlightOpts::builder()
			.foreground(&hsl(327, 12, 45))
			.nocombine(true)
			.build(),
	)?;

	api::set_hl(
		0,
		"@character.special.bash",
		&SetHighlightOpts::builder()
			.foreground(&hsl(290, 60, 60))
			.nocombine(true)
			.build(),
	)?;

	Ok(())
}
