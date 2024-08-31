use nvim_oxi::api::{self, opts::SetHighlightOpts};

pub fn load() -> Result<(), api::Error> {
	api::set_hl(
		0,
		"Normal",
		&SetHighlightOpts::builder()
			.background(&crate::color::BACKGROUND.to_rgb())
			.build(),
	)?;

	Ok(())
}
