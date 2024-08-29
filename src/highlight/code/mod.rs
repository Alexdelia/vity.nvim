mod comment;

use nvim_oxi::api;

pub fn load() -> Result<(), api::Error> {
	comment::load()?;

	Ok(())
}
