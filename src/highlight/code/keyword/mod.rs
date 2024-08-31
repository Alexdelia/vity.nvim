mod flow;

use nvim_oxi::api;

pub fn load() -> Result<(), api::Error> {
	flow::load()?;

	Ok(())
}
