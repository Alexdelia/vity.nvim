use nvim_oxi::api::{self, opts::SetHighlightOpts};

use crate::color::{BACKGROUND, BACKGROUND_D1, BACKGROUND_U2};
use crate::hsl;

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
		"NormalFloat",
		&SetHighlightOpts::builder()
			.background(&BACKGROUND_U2.to_rgb())
			.build(),
	)?;
	api::set_hl(
		0,
		"FloatBorder",
		&SetHighlightOpts::builder()
			.foreground(&hsl(0, 0, 50))
			.background(&BACKGROUND_U2.to_rgb())
			.build(),
	)?;

	api::set_hl(
		0,
		"Pmenu",
		&SetHighlightOpts::builder()
			.background(&BACKGROUND_D1.to_rgb())
			.build(),
	)?;
	api::set_hl(
		0,
		"PmenuBorder",
		&SetHighlightOpts::builder()
			.background(&BACKGROUND_D1.to_rgb())
			.build(),
	)?;
	api::set_hl(
		0,
		"StatusLine",
		&SetHighlightOpts::builder()
			.background(&BACKGROUND.to_rgb())
			.foreground(&hsl(0, 0, 75))
			.build(),
	)?;
	api::set_hl(
		0,
		"StatusLineNC",
		&SetHighlightOpts::builder()
			.background(&BACKGROUND.to_rgb())
			.foreground(&hsl(0, 0, 50))
			.build(),
	)?;

	Ok(())
}
