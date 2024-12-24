mod code;
mod common;
mod overlay;
mod plugin;
mod window;

use nvim_oxi::api;

pub fn load() -> Result<(), api::Error> {
	common::load()?;
	code::load()?;
	window::load()?;
	overlay::load()?;
	plugin::load()?;

	Ok(())
}
