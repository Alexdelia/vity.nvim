use nvim_oxi::api::{self, opts::SetHighlightOpts};

use crate::{
	color::{ERROR, HELP, INFO, SUCCESS, WARNING},
	hsl::convert::to_rgb::hsl,
};

const PRE: &str = "Diagnostic";
const VIRTUAL: &str = "Virtual";

const FOREGROUND_SAT: u8 = 95;
const FOREGROUND_LUM: u8 = 60;
const BACKGROUND_SAT: u8 = 25;
const BACKGROUND_LUM: u8 = 17;

pub(super) fn load() -> Result<(), api::Error> {
	for (name, color) in &[
		("Error", ERROR),
		("Warning", WARNING),
		("Warn", WARNING),
		("Info", INFO),
		("Hint", HELP),
		("Success", SUCCESS),
		("Ok", SUCCESS),
	] {
		api::set_hl(
			0,
			&format!("{PRE}{name}"),
			&SetHighlightOpts::builder()
				.foreground(&color.to_rgb())
				.build(),
		)?;
	}

	for (name, color_fg, color_bg) in &[
		(
			"Error",
			hsl(0, FOREGROUND_SAT, FOREGROUND_LUM + 5),
			hsl(0, BACKGROUND_SAT - 9, BACKGROUND_LUM + 1),
		),
		(
			"Warning",
			hsl(30, FOREGROUND_SAT, FOREGROUND_LUM),
			hsl(30, BACKGROUND_SAT, BACKGROUND_LUM),
		),
		(
			"Warn",
			hsl(30, FOREGROUND_SAT, FOREGROUND_LUM),
			hsl(30, BACKGROUND_SAT, BACKGROUND_LUM),
		),
		(
			"Info",
			hsl(INFO.h, FOREGROUND_SAT, FOREGROUND_LUM),
			hsl(INFO.h, BACKGROUND_SAT, BACKGROUND_LUM),
		),
		(
			"Hint",
			hsl(HELP.h, FOREGROUND_SAT, FOREGROUND_LUM),
			hsl(HELP.h, BACKGROUND_SAT, BACKGROUND_LUM),
		),
		(
			"Success",
			hsl(SUCCESS.h, FOREGROUND_SAT, FOREGROUND_LUM),
			hsl(SUCCESS.h, BACKGROUND_SAT, BACKGROUND_LUM),
		),
		(
			"Ok",
			hsl(SUCCESS.h, FOREGROUND_SAT, FOREGROUND_LUM),
			hsl(SUCCESS.h, BACKGROUND_SAT, BACKGROUND_LUM),
		),
	] {
		let hl = format!("{PRE}{VIRTUAL}Text{name}");
		api::set_hl(
			0,
			&hl,
			&SetHighlightOpts::builder()
				.foreground(color_fg)
				.background(color_bg)
				.build(),
		)?;

		api::set_hl(
			0,
			&format!("{PRE}{VIRTUAL}Lines{name}"),
			follow!(hl.as_str()),
		)?;
	}

	Ok(())
}
