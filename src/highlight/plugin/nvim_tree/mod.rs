mod git;

use nvim_oxi::api::{self, opts::SetHighlightOpts};

use const_format::formatcp as f;

use crate::hsl;

const PRE: &str = "NvimTree";

pub fn load() -> Result<(), api::Error> {
	git::load()?;

	api::set_hl(
		0,
		f!("{PRE}FolderIcon"),
		&SetHighlightOpts::builder()
			.foreground(&hsl(201, 15, 62))
			.bold(true)
			.build(),
	)?;

	Ok(())
}
