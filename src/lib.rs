mod hsl;

use nvim_oxi::{
	api::{self, opts::SetHighlightOpts},
	Dictionary, Function,
};

#[nvim_oxi::plugin]
fn vity() -> nvim_oxi::Result<Dictionary> {
	let load: Function<(), Result<(), api::Error>> = Function::from_fn(move |()| {
		api::set_hl(
			0,
			"Comment",
			&SetHighlightOpts::builder()
				.foreground(&hsl::hsl(0, 100, 50))
				.build(),
		)?;

		Ok(())
	});

	Ok(Dictionary::from_iter([
		("load", load.clone()),
		("setup", load.clone()),
		("colorscheme", load),
	]))
}
