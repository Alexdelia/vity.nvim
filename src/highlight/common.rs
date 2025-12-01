use nvim_oxi::api::{self, opts::SetHighlightOpts};

use crate::{
	color::BACKGROUND,
	hsl::{convert::to_rgb::hsl, Hsl},
};

pub fn load() -> Result<(), api::Error> {
	api::set_hl(
		0,
		"Directory",
		&SetHighlightOpts::builder()
			.foreground(&hsl(200, 80, 50))
			.build(),
	)?;

	api::set_hl(
		0,
		"NonText",
		&SetHighlightOpts::builder()
			.foreground(
				&(Hsl {
					h: BACKGROUND.h,
					s: BACKGROUND.s,
					l: BACKGROUND.l + 6,
				}
				.to_rgb()),
			)
			.build(),
	)?;

	Ok(())
}
