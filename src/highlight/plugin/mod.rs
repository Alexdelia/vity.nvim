mod cmp;
mod telescope;
// mod bufferline;

use nvim_oxi::api;

pub fn load() -> Result<(), api::Error> {
	telescope::load()?;
	cmp::load()?;
	// bufferline::load()?;

	Ok(())
}
