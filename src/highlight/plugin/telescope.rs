use nvim_oxi::api::{self, opts::SetHighlightOpts};

use const_format::formatcp as f;

use crate::{
	color::{BACKGROUND_U2, SELECTION},
	hsl,
};

const PRE: &str = "Telescope";

pub fn load() -> Result<(), api::Error> {
	api::set_hl(
		0,
		f!("{PRE}Matching"),
		&SetHighlightOpts::builder()
			.background(&hsl(0, 25, 25))
			.build(),
	)?;

	api::set_hl(
		0,
		f!("{PRE}Selection"),
		&SetHighlightOpts::builder()
			.background(&SELECTION.to_rgb())
			.build(),
	)?;

	api::set_hl(
		0,
		f!("{PRE}Border"),
		&SetHighlightOpts::builder()
			.background(&BACKGROUND_U2.to_rgb())
			.build(),
	)?;

	api::set_hl(
		0,
		f!("{PRE}PromptCounter"),
		&SetHighlightOpts::builder()
			.foreground(&hsl(0, 0, 50))
			.build(),
	)?;

	Ok(())
}
