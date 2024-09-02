use crate::Hsl;

pub const BACKGROUND: Hsl = Hsl { h: 0, s: 0, l: 12 };
pub const BACKGROUND_D1: Hsl = Hsl { h: 0, s: 0, l: 10 };
pub const BACKGROUND_D2: Hsl = Hsl { h: 0, s: 0, l: 8 };

pub const PRIMARY: Hsl = Hsl {
	h: 260,
	s: 100,
	l: 75,
};
pub const SECONDARY_HUE: Hsl = Hsl {
	h: 160,
	s: 100,
	l: 69,
};

pub const ERROR: Hsl = Hsl { h: 0, s: 65, l: 50 };
pub const WARNING: Hsl = Hsl {
	h: 60,
	s: 65,
	l: 50,
};
pub const SUCCESS: Hsl = Hsl {
	h: 120,
	s: 65,
	l: 50,
};
pub const INFO: Hsl = Hsl {
	h: 205,
	s: 65,
	l: 50,
};
pub const HELP: Hsl = Hsl {
	h: 180,
	s: 65,
	l: 50,
};
