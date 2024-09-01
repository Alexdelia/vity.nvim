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
			.foreground(&BACKGROUND_D1.to_rgb())
			.build(),
	)?;
	// inactive buffer
	api::set_hl(
		0,
		"BufferLineBackground",
		&SetHighlightOpts::builder()
			.foreground(&BACKGROUND_D1.to_rgb())
			.build(),
	)?;
	// active buffer
	api::set_hl(
		0,
		"BufferLineBufferVisible",
		&SetHighlightOpts::builder()
			.foreground(&BACKGROUND.to_rgb())
			.build(),
	)?;

	Ok(())
}
