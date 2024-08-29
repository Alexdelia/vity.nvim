use hsl::hsl;

/*
#[test]
fn no_compile() {
	hsl!("hey", 0, 0);
	hsl!(-1, 0, 0);
	hsl!(69_000, 0, 0);
	hsl!(0, 0);
	hsl!(360, 0, 0, some);
}
*/

#[test]
fn test_hsl_int() {
	assert_eq!(hsl!(0, 0, 0), "#000000");
	assert_eq!(hsl!(0, 0, 100), "#ffffff");
	assert_eq!(hsl!(0, 100, 50), "#ff0000");
	assert_eq!(hsl!(120, 100, 50), "#00ff00");
	assert_eq!(hsl!(240, 100, 50), "#0000ff");
	assert_eq!(hsl!(360, 100, 50), "#ff0000");
	assert_eq!(hsl!(42, 69, 82), "#d1a641");
	assert_eq!(hsl!(348, 28, 100), "#ffb7c5");
}

/*
#[test]
fn test_hsl_float() {
	assert_eq!(hsl!(0.0, 0.0, 0.0), "#000000");
	assert_eq!(hsl!(0.0, 0.0, 100.0), "#ffffff");
	assert_eq!(hsl!(0.0, 100.0, 50.0), "#ff0000");
	assert_eq!(hsl!(120.0, 100.0, 50.0), "#00ff00");
	assert_eq!(hsl!(240.0, 100.0, 50.0), "#0000ff");
	assert_eq!(hsl!(360.0, 100.0, 50.0), "#ff0000");
	assert_eq!(hsl!(42.0, 69.0, 82.0), "#d1a641");
	assert_eq!(hsl!(348.0, 28.0, 100.0), "#ffb7c5");
}

#[test]
fn test_hsl_mix() {
	assert_eq!(hsl!(0, 0.0, 0), "#000000");
	assert_eq!(hsl!(0, 0, 100.0), "#ffffff");
	assert_eq!(hsl!(0.0, 100, 50), "#ff0000");
	assert_eq!(hsl!(120, 100.0, 50), "#00ff00");
	assert_eq!(hsl!(240, 100, 50.0), "#0000ff");
	assert_eq!(hsl!(360.0, 100.0, 50), "#ff0000");
	assert_eq!(hsl!(42, 69.0, 82.0), "#d1a641");
	assert_eq!(hsl!(348, 28, 100.0), "#ffb7c5");
}
*/
