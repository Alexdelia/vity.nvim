use crate::Float;

/// Converts HSL to RGB.
///
/// # Arguments
/// * `h` - Hue (0.0..=1.0)
/// * `s` - Saturation (0.0..=1.0)
/// * `l` - Lightness (0.0..=1.0)
///
/// # Returns
/// * A tuple containing the RGB values as u8 (0..=255).
pub fn hsl_to_rgb(h: Float, s: Float, l: Float) -> (u8, u8, u8) {
	let q = if l < 0.5 {
		l * (1.0 + s)
	} else {
		l + s - l * s
	};
	let p = 2.0 * l - q;

	let r = hue_to_rgb(p, q, h + 1.0 / 3.0);
	let g = hue_to_rgb(p, q, h);
	let b = hue_to_rgb(p, q, h - 1.0 / 3.0);

	(
		(r * 255.0).round() as u8,
		(g * 255.0).round() as u8,
		(b * 255.0).round() as u8,
	)
}

fn hue_to_rgb(p: Float, q: Float, t: Float) -> Float {
	let t = if t < 0.0 {
		t + 1.0
	} else if t > 1.0 {
		t - 1.0
	} else {
		t
	};

	if t < 1.0 / 6.0 {
		p + (q - p) * 6.0 * t
	} else if t < 1.0 / 2.0 {
		q
	} else if t < 2.0 / 3.0 {
		p + (q - p) * (2.0 / 3.0 - t) * 6.0
	} else {
		p
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_hsl_to_rgb() {
		assert_eq!(hsl_to_rgb(0.0, 0.0, 0.0), (0, 0, 0));
		assert_eq!(hsl_to_rgb(0.0, 0.0, 1.0), (255, 255, 255));
		assert_eq!(hsl_to_rgb(0.0, 1.0, 0.5), (255, 0, 0));
		assert_eq!(hsl_to_rgb(1.0 / 3.0, 1.0, 0.5), (0, 255, 0));
		assert_eq!(hsl_to_rgb(2.0 / 3.0, 1.0, 0.5), (0, 0, 255));
		assert_eq!(hsl_to_rgb(1.0, 1.0, 0.5), (255, 0, 0));
		assert_eq!(hsl_to_rgb(0.4, 0.82, 0.69), (111, 241, 163));
	}
}
