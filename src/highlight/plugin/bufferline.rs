use nvim_oxi::api::{self, opts::SetHighlightOpts};

use crate::{
	color::{
		BACKGROUND, BACKGROUND_D1, BACKGROUND_D2, ERROR, HELP, INFO, PRIMARY, SUCCESS, WARNING,
	},
	hsl, Hsl,
};

use const_format::formatcp as f;

const PRE: &str = "BufferLine";

const K_ERR: &str = "Error";
const K_WARN: &str = "Warning";
const K_INFO: &str = "Info";
const K_HELP: &str = "Hint";

const BUF: &str = "Buffer";
const DIAG: &str = "Diagnostic";
const NUM: &str = "Numbers";
const PICK: &str = "Pick";
const INDIC: &str = "Indicator";
const CLOSE_BTN: &str = "CloseButton";
const TAB: &str = "Tab";
const DUP: &str = "Duplicate";
const MODI: &str = "Modified";

const SEL: &str = "Selected";
const VIS: &str = "Visible";

pub fn load() -> Result<(), api::Error> {
	// elevation 0 to 2
	let bg = (
		BACKGROUND_D2.to_rgb(), // back bar background
		BACKGROUND_D1.to_rgb(), // unselected buffer background
		BACKGROUND.to_rgb(),    // selected buffer background
	);
	let fg_unselected = (Hsl { h: 0, s: 0, l: 25 }).to_rgb();
	let fg_selected = PRIMARY.blend(&BACKGROUND, 0.7).to_rgb();

	let error = ERROR.to_rgb();
	let warning = WARNING.to_rgb();
	let success = SUCCESS.to_rgb();
	let info = INFO.to_rgb();
	let help = HELP.to_rgb();
	let gray = (Hsl { h: 0, s: 0, l: 50 }).to_rgb();

	// elevation 0
	api::set_hl(
		0,
		f!("{PRE}Fill"),
		&SetHighlightOpts::builder().background(&bg.0).build(),
	)?;

	// elevation 1
	for (group, fg) in &[
		(f!("{PRE}Background"), &fg_unselected),
		(f!("{PRE}{K_ERR}"), &error),
		(f!("{PRE}{K_WARN}"), &warning),
		(f!("{PRE}{K_INFO}"), &info),
		(f!("{PRE}{K_HELP}"), &help),
		(f!("{PRE}{NUM}"), &gray),
	] {
		api::set_hl(
			0,
			group,
			&SetHighlightOpts::builder()
				.background(&bg.1)
				.foreground(fg)
				.bold(false)
				.italic(false)
				.build(),
		)?;
	}
	api::set_hl(
		0,
		f!("{PRE}{PICK}"),
		&SetHighlightOpts::builder()
			.background(&bg.1)
			.foreground(&success)
			.bold(true)
			.italic(false)
			.build(),
	)?;
	// for (group, link) in &[
	// ] {
	// api::set_hl(0, group, &SetHighlightOpts::builder().link(link).build())?;
	// }

	// elevation 2
	for (group, fg) in &[
		(f!("{PRE}{BUF}{SEL}"), &fg_selected),
		(f!("{PRE}{K_ERR}{SEL}"), &error),
		(f!("{PRE}{K_WARN}{SEL}"), &warning),
		(f!("{PRE}{K_INFO}{SEL}"), &info),
		(f!("{PRE}{K_HELP}{SEL}"), &help),
		(f!("{PRE}{PICK}{SEL}"), &success),
		(f!("{PRE}{INDIC}{SEL}"), &PRIMARY.to_rgb()),
		(f!("{PRE}{BUF}{VIS}"), &gray),
	] {
		api::set_hl(
			0,
			group,
			&SetHighlightOpts::builder()
				.background(&bg.2)
				.foreground(fg)
				.bold(true)
				.italic(false)
				.build(),
		)?;
	}
	api::set_hl(
		0,
		f!("{PRE}{NUM}{SEL}"),
		&SetHighlightOpts::builder()
			.background(&bg.2)
			.foreground(&gray)
			.bold(false)
			.italic(false)
			.build(),
	)?;
	api::set_hl(
		0,
		f!("{PRE}{INDIC}{VIS}"),
		&SetHighlightOpts::builder()
			.background(&bg.2)
			.foreground(&bg.2)
			.build(),
	)?;
	for (link, groups) in &[
		// (f!("{PRE}{BUF}{SEL}"), vec![]),
		// (f!("{PRE}{K_ERR}{SEL}"), vec![]),
		// (f!("{PRE}{K_WARN}{SEL}"), vec![]),
		(
			f!("{PRE}{K_INFO}{SEL}"),
			vec![f!("{PRE}{MODI}{SEL}"), f!("{PRE}{MODI}{VIS}")],
		),
		// (f!("{PRE}{K_HELP}{SEL}"), vec![]),
		(f!("{PRE}{PICK}{SEL}"), vec![f!("{PRE}{PICK}{VIS}")]),
		(
			f!("{PRE}{BUF}{VIS}"),
			vec![
				f!("{PRE}{K_ERR}{VIS}"),
				f!("{PRE}{K_WARN}{VIS}"),
				f!("{PRE}{K_INFO}{VIS}"),
				f!("{PRE}{K_HELP}{VIS}"),
			],
		),
		(f!("{PRE}{NUM}{SEL}"), vec![f!("{PRE}{NUM}{VIS}")]),
	] {
		for group in groups {
			api::set_hl(0, group, &SetHighlightOpts::builder().link(link).build())?;
		}
	}

	// unsupported because I don't use them
	for (group, link) in &[
		(f!("{PRE}{TAB}"), f!("{PRE}{BUF}Background")),
		(f!("{PRE}{TAB}{SEL}"), f!("{PRE}{BUF}{SEL}")),
		(f!("{PRE}{DUP}{SEL}"), f!("{PRE}{BUF}{VIS}")),
		(f!("{PRE}{DUP}{VIS}"), f!("{PRE}{BUF}{VIS}")),
		(f!("{PRE}{DIAG}{SEL}"), "Normal"),
		(f!("{PRE}{DIAG}{VIS}"), f!("{PRE}{DIAG}{SEL}")),
		(f!("{PRE}{K_ERR}{DIAG}{SEL}"), f!("{PRE}{K_ERR}{SEL}")),
		(f!("{PRE}{K_WARN}{DIAG}{SEL}"), f!("{PRE}{K_WARN}{SEL}")),
		(f!("{PRE}{K_INFO}{DIAG}{SEL}"), f!("{PRE}{K_INFO}{SEL}")),
		(f!("{PRE}{K_HELP}{DIAG}{SEL}"), f!("{PRE}{K_HELP}{SEL}")),
		(f!("{PRE}{K_ERR}{DIAG}{VIS}"), f!("{PRE}{K_ERR}{VIS}")),
		(f!("{PRE}{K_WARN}{DIAG}{VIS}"), f!("{PRE}{K_WARN}{VIS}")),
		(f!("{PRE}{K_INFO}{DIAG}{VIS}"), f!("{PRE}{K_INFO}{VIS}")),
		(f!("{PRE}{K_HELP}{DIAG}{VIS}"), f!("{PRE}{K_HELP}{VIS}")),
		(f!("{PRE}{CLOSE_BTN}{SEL}"), f!("{PRE}{NUM}{SEL}")),
		(f!("{PRE}{CLOSE_BTN}{VIS}"), f!("{PRE}{NUM}{VIS}")),
	] {
		api::set_hl(0, group, &SetHighlightOpts::builder().link(link).build())?;
	}

	// TODO: separator

	Ok(())
}
