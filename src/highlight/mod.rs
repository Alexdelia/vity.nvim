mod code;
mod plugin;
mod window;

use nvim_oxi::api;

pub fn load() -> Result<(), api::Error> {
	code::load()?;
	window::load()?;
	plugin::load()?;

	Ok(())
}
