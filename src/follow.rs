use nvim_oxi::api::{opts::SetHighlightOpts, HlGroup};

pub fn follow<HL>(hl: HL) -> SetHighlightOpts
where
	HL: HlGroup,
{
	SetHighlightOpts::builder().link(hl).build()
}
