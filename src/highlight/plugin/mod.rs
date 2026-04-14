mod cmp;
mod dev_icon;
mod nvim_tree;
mod telescope;
// mod bufferline;

use nvim_oxi::api;

pub fn load() -> Result<(), api::Error> {
	dev_icon::load()?;
	telescope::load()?;
	cmp::load()?;
	nvim_tree::load()?;
	// bufferline::load()?;

	Ok(())
}
