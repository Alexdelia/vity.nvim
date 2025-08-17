use nvim_oxi::api::{self, opts::SetHighlightOpts};

use crate::{
	color::{ERROR, HELP, INFO, SUCCESS, WARNING},
	hsl::convert::to_rgb::hsl,
};

const PRE: &str = "Diagnostic";
const VIRTUAL: &str = "Virtual";

pub fn load() -> Result<(), api::Error> {
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

	let base_fg_sat = 95;
	let base_fg_lum = 60;

	let base_bg_sat = 25;
	let base_bg_lum = 17;

	for (name, color_fg, color_bg) in &[
		(
			"Error",
			hsl(0, base_fg_sat, base_fg_lum + 5),
			hsl(0, base_bg_sat - 9, base_bg_lum + 1),
		),
		(
			"Warning",
			hsl(30, base_fg_sat, base_fg_lum),
			hsl(30, base_bg_sat, base_bg_lum),
		),
		(
			"Warn",
			hsl(30, base_fg_sat, base_fg_lum),
			hsl(30, base_bg_sat, base_bg_lum),
		),
		(
			"Info",
			hsl(INFO.h, base_fg_sat, base_fg_lum),
			hsl(INFO.h, base_bg_sat, base_bg_lum),
		),
		(
			"Hint",
			hsl(HELP.h, base_fg_sat, base_fg_lum),
			hsl(HELP.h, base_bg_sat, base_bg_lum),
		),
		(
			"Success",
			hsl(SUCCESS.h, base_fg_sat, base_fg_lum),
			hsl(SUCCESS.h, base_bg_sat, base_bg_lum),
		),
		(
			"Ok",
			hsl(SUCCESS.h, base_fg_sat, base_fg_lum),
			hsl(SUCCESS.h, base_bg_sat, base_bg_lum),
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
