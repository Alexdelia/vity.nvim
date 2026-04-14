mod git;

use nvim_oxi::api;

const PRE: &str = "NvimTree";

pub fn load() -> Result<(), api::Error> {
	git::load()?;

	Ok(())
}
