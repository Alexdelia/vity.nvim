use nvim_oxi::api::{opts::SetHighlightOpts, HlGroup};

#[macro_export]
macro_rules! follow {
	($hl:expr) => {
		&SetHighlightOpts::builder().link($hl).build()
	};
}
