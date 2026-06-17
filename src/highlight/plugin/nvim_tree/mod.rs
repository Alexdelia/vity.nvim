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
			.foreground(&hsl(0, 0, 33))
			.bold(true)
			.build(),
	)?;
	api::set_hl(
		0,
		"{PRE}RootFolder",
		&SetHighlightOpts::builder()
			.foreground(&hsl(265, 75, 72))
			.bold(true)
			.build(),
	)?;

	api::set_hl(
		0,
		f!("{PRE}ExecFile"),
		&SetHighlightOpts::builder()
			.foreground(&hsl(108, 72, 49))
			.build(),
	)?;
	api::set_hl(
		0,
		f!("{PRE}Symlink"),
		&SetHighlightOpts::builder()
			.foreground(&hsl(190, 80, 65))
			.build(),
	)?;
	api::set_hl(0, f!("{PRE}SymlinkIcon"), follow!(f!("{PRE}Symlink")))?;

	api::set_hl(
		0,
		f!("{PRE}NormalFloat"),
		&SetHighlightOpts::builder().build(),
	)?;

	Ok(())
}
