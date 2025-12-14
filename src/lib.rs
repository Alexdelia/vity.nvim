#[macro_use]
mod follow;
mod hsl;

mod color;
mod highlight;

use hsl::{Hsl, convert::to_rgb::hsl};

use nvim_oxi::{Dictionary, Function, api};

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
