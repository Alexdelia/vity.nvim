mod blend;
pub mod convert;

type Float = f64;

type Hue = u16;
type Sat = u8;
type Lum = u8;

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Hsl {
	pub h: Hue,
	pub s: Sat,
	pub l: Lum,
}

impl Hsl {
	/*
	pub fn new(h: Hue, s: Sat, l: Lum) -> Self {
		Self { h, s, l }
	}
	*/

	pub fn to_rgb(&self) -> String {
		convert::to_rgb::hsl(self.h, self.s, self.l)
	}

	/// blend two HSL colors
	/// this works like transparency where *k* is the alpha/transparency of the first color
	///
	/// # Arguments
	/// * `other` - the other HSL color to blend with
	/// * `k` - the alpha/transparency of the first color (0.0..=1.0)
	///
	/// # Example
	/// ```
	/// use hsl::Hsl;
	///
	/// let fg = Hsl { h: 260, s: 80, l: 70 };
	/// let bg = Hsl { h: 0, s: 0, l: 25 };
	///
	/// let result = fg.blend(&bg, 0.7); // 70% transparent -> 30% fg, 70% bg
	/// assert_eq!(result, Hsl { h: 259, s: 19, l: 39 });
	/// ```
	pub fn blend(&self, other: &Hsl, k: f32) -> Hsl {
		blend::blend(self, other, k)
	}
}
