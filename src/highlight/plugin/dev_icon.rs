use nvim_oxi::api::{self, opts::SetHighlightOpts};

use const_format::formatcp as f;

const PRE: &str = "DevIcon";

pub fn load() -> Result<(), api::Error> {
	api::set_hl(
		0,
		f!("{PRE}Sh"),
		&SetHighlightOpts::builder().link(f!("{PRE}Shell")).build(),
	)?;

	Ok(())
}
