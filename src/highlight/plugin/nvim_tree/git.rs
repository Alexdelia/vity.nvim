use nvim_oxi::api::{self, opts::SetHighlightOpts};

use const_format::formatcp as f;

use crate::hsl;

use super::PRE as PRE_BASE;

const PRE: &str = f!("{PRE_BASE}Git");

pub fn load() -> Result<(), api::Error> {
	api::set_hl(
		0,
		f!("{PRE}DirtyIcon"),
		&SetHighlightOpts::builder()
			.foreground(&hsl(69, 70, 48))
			.build(),
	)?;
	api::set_hl(
		0,
		f!("{PRE}FolderDirtyHL"),
		&SetHighlightOpts::builder().italic(true).build(),
	)?;
	api::set_hl(
		0,
		f!("{PRE}FileDirtyHL"),
		&SetHighlightOpts::builder().italic(true).build(),
	)?;

	api::set_hl(
		0,
		f!("{PRE}StagedIcon"),
		&SetHighlightOpts::builder()
			.foreground(&hsl(69, 70, 32))
			.build(),
	)?;
	api::set_hl(
		0,
		f!("{PRE}FolderStagedHL"),
		&SetHighlightOpts::builder().underdotted(true).build(),
	)?;
	api::set_hl(
		0,
		f!("{PRE}FileStagedHL"),
		&SetHighlightOpts::builder().underdotted(true).build(),
	)?;

	api::set_hl(
		0,
		f!("{PRE}NewIcon"),
		&SetHighlightOpts::builder()
			.foreground(&hsl(110, 50, 40))
			.build(),
	)?;
	api::set_hl(
		0,
		f!("{PRE}FolderNewHL"),
		&SetHighlightOpts::builder().underdashed(true).build(),
	)?;
	api::set_hl(
		0,
		f!("{PRE}FileNewHL"),
		&SetHighlightOpts::builder().underdashed(true).build(),
	)?;

	api::set_hl(0, f!("{PRE}RenamedIcon"), follow!(f!("{PRE}StagedIcon")))?;
	api::set_hl(
		0,
		f!("{PRE}FolderRenamedHL"),
		follow!(f!("{PRE}FolderStagedHL")),
	)?;
	api::set_hl(
		0,
		f!("{PRE}FileRenamedHL"),
		follow!(f!("{PRE}FileStagedHL")),
	)?;

	api::set_hl(
		0,
		f!("{PRE}DeletedIcon"),
		&SetHighlightOpts::builder()
			.foreground(&hsl(0, 70, 40))
			.build(),
	)?;
	api::set_hl(
		0,
		f!("{PRE}FolderDeletedHL"),
		&SetHighlightOpts::builder().underdashed(true).build(),
	)?;
	api::set_hl(
		0,
		f!("{PRE}FileDeletedHL"),
		&SetHighlightOpts::builder().underdashed(true).build(),
	)?;

	Ok(())
}
