mod comment;
mod delimiter;
mod identifier;
mod keyword;
mod literal;
mod namespace;
mod r#type;

use nvim_oxi::api;

pub fn load() -> Result<(), api::Error> {
	literal::load()?;
	r#type::load()?;
	identifier::load()?;
	keyword::load()?;
	delimiter::load()?;
	comment::load()?;
	namespace::load()?;

	Ok(())
}
