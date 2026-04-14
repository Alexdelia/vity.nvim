use nvim_oxi::api::{self, opts::SetHighlightOpts};

use const_format::formatcp as f;

use crate::hsl;

use super::PRE as PRE_BASE;

const PRE: &str = f!("{PRE_BASE}Git");

pub fn load() -> Result<(), api::Error> {
	let follow_dir = follow!("Directory");
	let disable = follow!("NONE");

	api::set_hl(
		0,
		f!("{PRE}DirtyIcon"),
		&SetHighlightOpts::builder()
			.foreground(&hsl(69, 70, 48))
			.build(),
	)?;
	api::set_hl(0, f!("{PRE}FolderDirtyHL"), follow_dir)?;
	api::set_hl(0, f!("{PRE}FileDirtyHL"), disable)?;

	api::set_hl(
		0,
		f!("{PRE}StagedIcon"),
		&SetHighlightOpts::builder()
			.foreground(&hsl(69, 70, 32))
			.build(),
	)?;
	api::set_hl(0, f!("{PRE}FolderStagedHL"), follow_dir)?;
	api::set_hl(0, f!("{PRE}FileStagedHL"), disable)?;

	api::set_hl(
		0,
		f!("{PRE}NewIcon"),
		&SetHighlightOpts::builder()
			.foreground(&hsl(110, 50, 40))
			.build(),
	)?;
	api::set_hl(0, f!("{PRE}FolderNewHL"), follow_dir)?;
	api::set_hl(0, f!("{PRE}FileNewHL"), disable)?;

	api::set_hl(0, f!("{PRE}RenamedIcon"), follow!(f!("{PRE}StagedIcon")))?;
	api::set_hl(0, f!("{PRE}FolderRenamedHL"), follow_dir)?;
	api::set_hl(0, f!("{PRE}FileRenamedHL"), disable)?;

	api::set_hl(
		0,
		f!("{PRE}DeletedIcon"),
		&SetHighlightOpts::builder()
			.foreground(&hsl(0, 70, 40))
			.build(),
	)?;
	api::set_hl(0, f!("{PRE}FolderDeletedHL"), follow_dir)?;
	api::set_hl(0, f!("{PRE}FileDeletedHL"), disable)?;

	Ok(())
}
