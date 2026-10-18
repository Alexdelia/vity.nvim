use crate::hsl::{Float, Hue, Lum, Sat};

/// Generates a hex color string from HSL values.
///
/// # Arguments
/// * `h` - Hue (0..=360)
/// * `s` - Saturation (0..=100)
/// * `l` - Lightness (0..=100)
///
/// # Returns
/// * a hex color string (e.g. "#ff0000").
pub(crate) fn hsl(hue: Hue, sat: Sat, lum: Lum) -> String {
	let (red, green, blue) = hsl_to_rgb(
		Float::from(hue) / 360.0,
		Float::from(sat) / 100.0,
		Float::from(lum) / 100.0,
	);

	format!("#{red:02x}{green:02x}{blue:02x}")
}

/// converts HSL to RGB.
///
/// # Arguments
/// * `h` - Hue (0.0..=1.0)
/// * `s` - Saturation (0.0..=1.0)
/// * `l` - Lightness (0.0..=1.0)
///
/// # Returns
/// * a tuple containing the RGB values as u8 (0..=255).
pub(crate) fn hsl_to_rgb(hue: Float, sat: Float, lum: Float) -> (u8, u8, u8) {
	let q = if lum < 0.5 {
		lum * (1.0 + sat)
	} else {
		lum.mul_add(-sat, lum + sat)
	};
	let p = lum.mul_add(2.0, -q);

	(
		to_channel(hue_to_rgb(p, q, hue + 1.0 / 3.0)),
		to_channel(hue_to_rgb(p, q, hue)),
		to_channel(hue_to_rgb(p, q, hue - 1.0 / 3.0)),
	)
}

#[expect(
	clippy::cast_possible_truncation,
	clippy::cast_sign_loss,
	reason = "value is clamped to 0..=255"
)]
fn to_channel(ratio: Float) -> u8 {
	(ratio * 255.0).round().clamp(0.0, 255.0) as u8
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
		((q - p) * 6.0).mul_add(t, p)
	} else if t < 1.0 / 2.0 {
		q
	} else if t < 2.0 / 3.0 {
		((q - p) * (2.0 / 3.0 - t)).mul_add(6.0, p)
	} else {
		p
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_hsl() {
		assert_eq!(hsl(0, 0, 0), "#000000");
		assert_eq!(hsl(0, 0, 100), "#ffffff");
		assert_eq!(hsl(0, 100, 50), "#ff0000");
		assert_eq!(hsl(120, 100, 50), "#00ff00");
		assert_eq!(hsl(240, 100, 50), "#0000ff");
		assert_eq!(hsl(360, 100, 50), "#ff0000");
		assert_eq!(hsl(42, 69, 82), "#f1deb1");
		assert_eq!(hsl(348, 100, 86), "#ffb8c6");
	}

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
