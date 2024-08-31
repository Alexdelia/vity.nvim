mod comment;
mod identifier;
mod keyword;
mod literal;
mod r#type;

use nvim_oxi::api;

pub fn load() -> Result<(), api::Error> {
	literal::load()?;
	r#type::load()?;
	identifier::load()?;
	keyword::load()?;
	comment::load()?;

	Ok(())
}
