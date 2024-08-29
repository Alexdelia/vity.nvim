/// Generates a hex color string from HSL values.
///
/// # Arguments
/// * `h` - Hue (0..=360)
/// * `s` - Saturation (0..=100)
/// * `l` - Lightness (0..=100)
///
/// # Returns
/// * a hex color string (e.g. "#ff0000").
pub fn hsl(h: u16, s: u16, l: u16) -> String {
	let (r, g, b) = hsl_to_rgb(h as Float / 360.0, s as Float / 100.0, l as Float / 100.0);

	format!("#{:02x}{:02x}{:02x}", r, g, b)
}

type Float = f64;

/// converts HSL to RGB.
///
/// # Arguments
/// * `h` - Hue (0.0..=1.0)
/// * `s` - Saturation (0.0..=1.0)
/// * `l` - Lightness (0.0..=1.0)
///
/// # Returns
/// * a tuple containing the RGB values as u8 (0..=255).
fn hsl_to_rgb(h: Float, s: Float, l: Float) -> (u8, u8, u8) {
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
