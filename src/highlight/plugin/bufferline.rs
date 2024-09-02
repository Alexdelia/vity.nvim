use nvim_oxi::api::{self, opts::SetHighlightOpts};

use crate::{
	color::{BACKGROUND, BACKGROUND_D1, PRIMARY_HUE},
	hsl,
};

pub fn load() -> Result<(), api::Error> {
	// elevation 0 to 2
	let bg = (
		BACKGROUND_D1.to_rgb(), // back bar background
		BACKGROUND_D1.to_rgb(), // unselected buffer background
		BACKGROUND.to_rgb(),    // selected buffer background
	);
	let fg_unselected = hsl(0, 0, 25);
	// let fg_selected = Hsl

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
