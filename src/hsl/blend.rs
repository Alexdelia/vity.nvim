use super::{
	convert::{from_rgb::from_rgb, to_rgb::hsl_to_rgb},
	Float, Hsl, Hue, Lum, Sat,
};

pub fn blend(c0: &Hsl, c1: &Hsl, k: f32) -> Hsl {
	let k1 = k.clamp(0.0, 1.0);
	let k0 = 1.0 - k1;

	let c0 = hsl_to_rgb(
		c0.h as Float / 360.0,
		c0.s as Float / 100.0,
		c0.l as Float / 100.0,
	);
	let c1 = hsl_to_rgb(
		c1.h as Float / 360.0,
		c1.s as Float / 100.0,
		c1.l as Float / 100.0,
	);

	let (r, g, b) = (
		(c0.0 as f32 * k0 + c1.0 as f32 * k1).round() as u8,
		(c0.1 as f32 * k0 + c1.1 as f32 * k1).round() as u8,
		(c0.2 as f32 * k0 + c1.2 as f32 * k1).round() as u8,
	);

	from_rgb(r, g, b)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_blend() {
		let fg = Hsl {
			h: 260,
			s: 80,
			l: 70,
		};
		let bg = Hsl { h: 0, s: 0, l: 25 };

		let result = blend(&fg, &bg, 0.7);
		assert_eq!(
			result,
			Hsl {
				h: 259,
				s: 18,
				l: 38
			}
		);
	}
}
