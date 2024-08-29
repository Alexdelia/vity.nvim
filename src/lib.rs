use const_format::formatcp as __formatcp;

#[cfg(test)]
mod test_proc_macro;

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
			&SetHighlightOpts::builder().foreground("#ff0000").build(),
		)?;

		// println!("{}", rgb!(255, 0, 0));

		Ok(())
	});

	Ok(Dictionary::from_iter([
		("load", load.clone()),
		("setup", load.clone()),
		("colorscheme", load),
	]))
}
