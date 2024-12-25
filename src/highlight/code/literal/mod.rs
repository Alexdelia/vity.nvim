mod alpha;
mod num;
mod symbol;
mod word;

use nvim_oxi::api;

pub fn load() -> Result<(), api::Error> {
	word::load()?;
	num::load()?;
	alpha::load()?;
	symbol::load()?;

	Ok(())
}
