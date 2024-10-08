use crate::hsl;

use nvim_oxi::api::{self, opts::SetHighlightOpts};

pub fn load() -> Result<(), api::Error> {
	api::set_hl(
		0,
		"Boolean",
		&SetHighlightOpts::builder()
			.foreground(&hsl(227, 63, 66))
			.build(),
	)?;

	let null = SetHighlightOpts::builder()
		.foreground(&hsl(293, 77, 52))
		.build();
	api::set_hl(0, "javaScriptNull", &null)?;
	api::set_hl(0, "yamlNull", &null)?;

	Ok(())
}
