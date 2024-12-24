mod code;
mod overlay;
mod plugin;
mod window;

use nvim_oxi::api;

pub fn load() -> Result<(), api::Error> {
	code::load()?;
	window::load()?;
	overlay::load()?;
	plugin::load()?;

	Ok(())
}
