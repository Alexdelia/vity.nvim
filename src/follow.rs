#[macro_export]
macro_rules! follow {
	($hl:expr) => {
		&SetHighlightOpts::builder().link($hl).build()
	};
}
