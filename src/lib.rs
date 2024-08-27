use nvim_oxi::{
	api::{
		self,
		opts::{NotifyOpts, SetHighlightOpts},
	},
	Dictionary, Function,
};

#[nvim_oxi::plugin]
fn vity() -> nvim_oxi::Result<Dictionary> {
	let load: Function<(), Result<(), api::Error>> = Function::from_fn(move |()| {
		api::set_hl(
			0,
			"Comment",
			&SetHighlightOpts::builder().foreground("#ff0000").build(),
		)?;

		api::notify(
			"vity loaded",
			api::types::LogLevel::Info,
			&NotifyOpts::default(),
		)?; // debug purpose
		Ok(())
	});

	Ok(Dictionary::from_iter([
		("load", load.clone()),
		("setup", load.clone()),
		("colorscheme", load),
	]))
}
