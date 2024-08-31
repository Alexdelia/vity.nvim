mod code;
mod window;

use nvim_oxi::api;

pub fn load() -> Result<(), api::Error> {
	code::load()?;
	window::load()?;

	Ok(())
}
