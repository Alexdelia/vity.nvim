use nvim_oxi::api::{self, opts::SetHighlightOpts};

#[nvim_oxi::test]
fn set_hl_does_not_abort_neovim() {
	let opts = SetHighlightOpts::builder()
		.foreground("#3399cc")
		.bold(true)
		.build();
	api::set_hl(0, "Directory", &opts).unwrap();
}
