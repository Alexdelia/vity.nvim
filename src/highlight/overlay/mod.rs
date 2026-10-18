mod diagnostic;
mod inlay_hint;

use nvim_oxi::api;

pub(super) fn load() -> Result<(), api::Error> {
	diagnostic::load()?;
	inlay_hint::load()?;

	Ok(())
}
