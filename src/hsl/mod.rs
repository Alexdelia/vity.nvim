pub mod convert;

pub struct Hsl {
	pub h: u16,
	pub s: u8,
	pub l: u8,
}

impl Hsl {
	/*
	pub fn new(h: u16, s: u8, l: u8) -> Self {
		Self { h, s, l }
	}
	*/

	pub fn to_rgb(&self) -> String {
		convert::hsl(self.h, self.s, self.l)
	}
}
