mod comment;
mod literal;
mod r#type;

use nvim_oxi::api;

pub fn load() -> Result<(), api::Error> {
	literal::load()?;
	r#type::load()?;
	comment::load()?;

	Ok(())
}
