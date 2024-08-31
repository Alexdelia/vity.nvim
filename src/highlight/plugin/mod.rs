mod bufferline;

use nvim_oxi::api;

pub fn load() -> Result<(), api::Error> {
	bufferline::load()?;

	Ok(())
}
