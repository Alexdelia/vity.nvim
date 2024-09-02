use nvim_oxi::api::{self, opts::SetHighlightOpts};

use crate::{
	color::{BACKGROUND, BACKGROUND_D1, BACKGROUND_D2, ERROR, INFO, PRIMARY, SUCCESS, WARNING},
	hsl,
};

use const_format::formatcp as f;

const PRE: &str = "BufferLine";

const ERR: &str = "Error";
const WARN: &str = "Warning";
const INFO: &str = "Info";
const HINT: &str = "Hint";

const SEL: &str = "Selected";

pub fn load() -> Result<(), api::Error> {
	// elevation 0 to 2
	let bg = (
		BACKGROUND_D2.to_rgb(), // back bar background
		BACKGROUND_D1.to_rgb(), // unselected buffer background
		BACKGROUND.to_rgb(),    // selected buffer background
	);
	let fg_unselected = (Hsl { h: 0, s: 0, l: 25 }).to_rgb();
	let fg_selected = PRIMARY.blend(bg.2, 0.7).to_rgb();

	let error = ERROR.to_rgb();
	let warning = WARNING.to_rgb();
	let success = SUCCESS.to_rgb();
	let info = INFO.to_rgb();
	let hint = (Hsl { h: 0, s: 0, l: 50 }).to_rgb();

	// elevation 0
	api::set_hl(
		0,
		f!("{PRE}Fill"),
		&SetHighlightOpts::builder().background(&bg.0).build(),
	)?;

	// elevation 1
	for (group, fg) in &[
		(f!("{PRE}Background"), &fg_unselected),
		(f!("{PRE}{ERR}"), &error),
		(f!("{PRE}{WARN}"), &warning),
		(f!("{PRE}{INFO}"), &info),
		(f!("{PRE}{HINT}"), &hint),
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
	for (group, link) in &[] {
		api::set_hl(0, group, &SetHighlightOpts::builder().link(link).build())?;
	}

	// elevation 2
	for (group, fg) in &[
		(f!("{PRE}Buffer{SEL}"), &fg_selected),
		(f!("{PRE}{ERR}{SEL}"), &error),
		(f!("{PRE}{WARN}{SEL}"), &warning),
		(f!("{PRE}{INFO}{SEL}"), &info),
		(f!("{PRE}{HINT}{SEL}"), &hint),
		(f!("{PRE}Pick{SEL}"), &success),
        (f!("{PRE}Indicator{SEL}"), &PRIMARY.to_rgb(),
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
	for (link, groups) in &[
        // (f!("{PRE}Buffer{SEL}"), &[]),
        // (f!("{PRE}{ERR}{SEL}"), &[]),
        // (f!("{PRE}{WARN}{SEL}"), &[]),
        (f!("{PRE}{INFO}{SEL}"), &[f!("{PRE}Modified{SEL}"])),
        (f!("{PRE}{HINT}{SEL}"), &[f!("{PRE}Numbers{SEL}"])),
        // (f!("{PRE}Pick{SEL}"), &[]),
	] {
		for group in groups {
			api::set_hl(0, group, &SetHighlightOpts::builder().link(link).build())?;
		}
	}

	// unsupported because I don't use them
	for (group, link) in &[
        (f!("{PRE}Tab"), f!("{PRE}BufferBackground")),
        (f!("{PRE}Tab{SEL}"), f!("{PRE}Buffer{SEL}")),
        (f!("{PRE}Duplicate{SEL}"), f!("{PRE}{HINT}{SEL}")),
	] {
		api::set_hl(0, group, &SetHighlightOpts::builder().link(link).build())?;
	}

    // TODO: separator

	Ok(())
}
