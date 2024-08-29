mod code;

use nvim_oxi::api;

pub fn load() -> Result<(), api::Error> {
	code::load()?;

	Ok(())
}
