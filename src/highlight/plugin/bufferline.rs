use nvim_oxi::api::{self, opts::SetHighlightOpts};

use crate::{
	color::{BACKGROUND, BACKGROUND_D1},
	hsl,
};

pub fn load() -> Result<(), api::Error> {
	// right empty bar
	api::set_hl(
		0,
		"BufferLineFill",
		&SetHighlightOpts::builder()
			.background(&BACKGROUND_D1.to_rgb())
			.build(),
	)?;
	// inactive buffer
	api::set_hl(
		0,
		"BufferLineBackground",
		&SetHighlightOpts::builder()
			.background(&BACKGROUND_D1.to_rgb())
			.build(),
	)?;
	// active buffer
	api::set_hl(
		0,
		"BufferLineBufferVisible",
		&SetHighlightOpts::builder()
			.background(&BACKGROUND.to_rgb())
			.build(),
	)?;
	// selected buffer
	api::set_hl(
		0,
		"BufferLineBufferSelected",
		&SetHighlightOpts::builder()
			.background(&BACKGROUND.to_rgb())
			.italic(false)
			.bold(false)
			.build(),
	)?;

	Ok(())
}
