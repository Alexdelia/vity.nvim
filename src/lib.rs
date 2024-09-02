mod color;
mod highlight;
mod hsl;

use hsl::{convert::to_rgb::hsl, Hsl};

use nvim_oxi::{api, Dictionary, Function};

#[nvim_oxi::plugin]
fn vity() -> nvim_oxi::Result<Dictionary> {
	let load: Function<(), Result<(), api::Error>> = Function::from_fn(move |()| {
		highlight::load()?;

		Ok(())
	});

	Ok(Dictionary::from_iter([
		("load", load.clone()),
		("setup", load.clone()),
		("colorscheme", load),
	]))
}
