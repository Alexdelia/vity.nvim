use crate::Hsl;

pub(crate) const BACKGROUND_U2: Hsl = Hsl { h: 0, s: 0, l: 16 };
// pub const BACKGROUND_U1: Hsl = Hsl { h: 0, s: 0, l: 14 };
pub(crate) const BACKGROUND: Hsl = Hsl { h: 0, s: 0, l: 12 };
pub(crate) const BACKGROUND_D1: Hsl = Hsl { h: 0, s: 0, l: 10 };
// pub const BACKGROUND_D2: Hsl = Hsl { h: 0, s: 0, l: 8 };

pub(crate) const SELECTION: Hsl = Hsl { h: 0, s: 0, l: 22 };

/*
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
*/

pub(crate) const ERROR: Hsl = Hsl { h: 0, s: 95, l: 65 };
pub(crate) const WARNING: Hsl = Hsl {
	h: 30,
	s: 95,
	l: 60,
};
pub(crate) const SUCCESS: Hsl = Hsl {
	h: 120,
	s: 65,
	l: 50,
};
pub(crate) const INFO: Hsl = Hsl {
	h: 205,
	s: 65,
	l: 50,
};
pub(crate) const HELP: Hsl = Hsl {
	h: 180,
	s: 65,
	l: 50,
};

// 280°, 16%, 32% dark normal fg
