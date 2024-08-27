use nvim_oxi::{api, print, Dictionary, Function};

#[nvim_oxi::plugin]
fn vity() -> nvim_oxi::Result<Dictionary> {
	let load: Function<(), Result<(), api::Error>> = Function::from_fn(move |()| {
		print!("vity loaded"); // debug purpose

		Ok(())
	});

	Ok(Dictionary::from_iter([
		("load", load.clone()),
		("setup", load.clone()),
		("colorscheme", load),
	]))
}
