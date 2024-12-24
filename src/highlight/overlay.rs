use nvim_oxi::api::{self, opts::SetHighlightOpts};

use crate::color::{ERROR, HELP, INFO, SUCCESS, WARNING};

const PRE: &str = "Diagnostic";

pub fn load() -> Result<(), api::Error> {
	for (name, color) in &[
		("Error", ERROR),
		("Warning", WARNING),
		("Info", INFO),
		("Hint", HELP),
		("Success", SUCCESS),
	] {
		api::set_hl(
			0,
			&format!("{PRE}{name}"),
			&SetHighlightOpts::builder()
				.foreground(&color.to_rgb())
				.build(),
		)?;
	}

	Ok(())
}
