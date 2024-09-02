use crate::hsl::{Hsl, Hue, Lum, Sat};

pub fn from_rgb(r: u8, g: u8, b: u8) -> Hsl {
	let r = r as f64 / 255.0;
	let g = g as f64 / 255.0;
	let b = b as f64 / 255.0;

	let cmax = r.max(g).max(b);
	let cmin = r.min(g).min(b);
	let delta = cmax - cmin;

	let hue = if delta == 0.0 {
		0.0
	} else if cmax == r {
		60.0 * (((g - b) / delta) % 6.0)
	} else if cmax == g {
		60.0 * (((b - r) / delta) + 2.0)
	} else {
		60.0 * (((r - g) / delta) + 4.0)
	};

	let lum = (cmax + cmin) / 2.0;

	let sat = if delta == 0.0 {
		0.0
	} else {
		delta / (1.0 - (2.0 * lum - 1.0).abs())
	};

	Hsl {
		h: hue as Hue,
		s: (sat * 100.0) as Sat,
		l: (lum * 100.0) as Lum,
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::hsl::Hsl;

	#[test]
	fn test_from_rgb() {
		assert_eq!(from_rgb(0, 0, 0), Hsl { h: 0, s: 0, l: 0 });
		assert_eq!(from_rgb(255, 255, 255), Hsl { h: 0, s: 0, l: 100 });
		assert_eq!(
			from_rgb(255, 0, 0),
			Hsl {
				h: 0,
				s: 100,
				l: 50
			}
		);
		assert_eq!(
			from_rgb(0, 255, 0),
			Hsl {
				h: 120,
				s: 100,
				l: 50
			}
		);
		assert_eq!(
			from_rgb(0, 0, 255),
			Hsl {
				h: 240,
				s: 100,
				l: 50
			}
		);
		assert_eq!(
			from_rgb(111, 241, 163),
			Hsl {
				h: 144,
				s: 82,
				l: 69
			}
		);
	}
}
