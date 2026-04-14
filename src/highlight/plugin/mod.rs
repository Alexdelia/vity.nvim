mod cmp;
mod dev_icon;
mod telescope;
// mod bufferline;

use nvim_oxi::api;

pub fn load() -> Result<(), api::Error> {
	dev_icon::load()?;
	telescope::load()?;
	cmp::load()?;
	// bufferline::load()?;

	Ok(())
}
