use nvim_oxi::api::{self, opts::SetHighlightOpts};

use crate::{
	color::{BACKGROUND, BACKGROUND_D1},
	hsl,
};

pub fn load() -> Result<(), api::Error> {
	api::set_hl(
		0,
		"Normal",
		&SetHighlightOpts::builder()
			.background(&BACKGROUND.to_rgb())
			.build(),
	)?;
	api::set_hl(
		0,
		"Pmenu",
		&SetHighlightOpts::builder()
			.background(&BACKGROUND_D1.to_rgb())
			.build(),
	)?;

	Ok(())
}
