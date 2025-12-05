use crate::{
	suhore_valsi::{SuhoreValsi_Slice, SuhoreValsi_Vec},
	tcita::Tcita_ti,
	valsi::{TiValsiLaLojban_Slice, TiValsiLaLojban_Vec, ValsiSlice},
};
use stodi::Stodi;

/// Full name: pa lerfu la lojban
/// TODO LOJBAN is there an official lujvo for this already?
///
/// Lojban character literal stored in memory as a UTF-8 compatible code point using [char]
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct Lerfu(char);

impl Tcita_ti for Lerfu {
	fn full_abstract_tcita_ti() -> SuhoreValsi_Vec {
		let suho_valsi = "pa lerfu la lojban"
			.split(" ")
			.map(|suho_lerfu| ValsiSlice::new(&suho_lerfu).unwrap());
		SuhoreValsi_Vec::new(suho_valsi).unwrap()
	}
}

impl Lerfu {
	pub const ALLOWED: [char; 26] = [
		'\'', ',', '.', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'r',
		's', 't', 'u', 'v', 'x', 'y', 'z',
	];
}

impl Stodi for Lerfu {
	fn check_stodi(&self) -> bool {
		Self::ALLOWED.contains(&self.0)
	}
}

impl Lerfu {
	pub fn new(c: char) -> Option<Self> {
		let ret = unsafe { Lerfu::new_unchecked(c) };
		ret.check_stodi().then_some(ret)
	}
}

/// Implementation understanding methdos
impl Lerfu {
	pub const unsafe fn new_unchecked(c: char) -> Self {
		Lerfu(c)
	}

	pub const fn char(&self) -> &char {
		&self.0
	}

	/// TODO: Don't use transmute
	pub const fn from_ref(char: &char) -> &Self {
		unsafe { std::mem::transmute(char) }
	}
}

impl std::fmt::Display for Lerfu {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.char())
	}
}

impl PartialEq for Lerfu {
	#[stodi::invariant(self.check_stodi())]
	#[stodi::invariant(other.check_stodi())]
	fn eq(&self, other: &Self) -> bool {
		self.0.eq_ignore_ascii_case(&other.0)
	}
}

#[test]
fn lerfu_macro() {
	assert_eq!(lerfu!(h), lerfu!('\''));
	assert_eq!(lerfu!(h), Lerfu::new('\'').unwrap());

	assert_eq!(lerfu!(a), Lerfu::new('a').unwrap());
	assert_eq!(lerfu!('a'), Lerfu::new('a').unwrap());
}

/// ```rust
/// use ah_lojban::lerfu::{Lerfu, lerfu};
/// assert_eq!(lerfu!(h), lerfu!('\''));
/// assert_eq!(lerfu!(h), Lerfu::new('\'').unwrap());
///
/// assert_eq!(lerfu!(a), Lerfu::new('a').unwrap());
/// ```
#[macro_export]
macro_rules! lerfu {
	('\'') => {
		unsafe { $crate::lerfu::Lerfu::new_unchecked('\'') }
	};
	(h) => {
		unsafe { $crate::lerfu::Lerfu::new_unchecked('\'') }
	};

	// normal from here
	(.) => {
		unsafe { $crate::lerfu::Lerfu::new_unchecked('.') }
	};
	(,) => {
		unsafe { $crate::lerfu::Lerfu::new_unchecked(',') }
	};
	(a) => {
		unsafe { $crate::lerfu::Lerfu::new_unchecked('a') }
	};
	(b) => {
		unsafe { $crate::lerfu::Lerfu::new_unchecked('b') }
	};
	(c) => {
		unsafe { $crate::lerfu::Lerfu::new_unchecked('c') }
	};
	(d) => {
		unsafe { $crate::lerfu::Lerfu::new_unchecked('d') }
	};
	(e) => {
		unsafe { $crate::lerfu::Lerfu::new_unchecked('e') }
	};
	(f) => {
		unsafe { $crate::lerfu::Lerfu::new_unchecked('f') }
	};
	(g) => {
		unsafe { $crate::lerfu::Lerfu::new_unchecked('g') }
	};
	(h) => {
		unsafe { $crate::lerfu::Lerfu::new_unchecked('h') }
	};
	(i) => {
		unsafe { $crate::lerfu::Lerfu::new_unchecked('i') }
	};
	(j) => {
		unsafe { $crate::lerfu::Lerfu::new_unchecked('j') }
	};
	(k) => {
		unsafe { $crate::lerfu::Lerfu::new_unchecked('k') }
	};
	(l) => {
		unsafe { $crate::lerfu::Lerfu::new_unchecked('l') }
	};
	(n) => {
		unsafe { $crate::lerfu::Lerfu::new_unchecked('n') }
	};
	(o) => {
		unsafe { $crate::lerfu::Lerfu::new_unchecked('o') }
	};
	(p) => {
		unsafe { $crate::lerfu::Lerfu::new_unchecked('p') }
	};
	(r) => {
		unsafe { $crate::lerfu::Lerfu::new_unchecked('r') }
	};
	(s) => {
		unsafe { $crate::lerfu::Lerfu::new_unchecked('s') }
	};
	(t) => {
		unsafe { $crate::lerfu::Lerfu::new_unchecked('t') }
	};
	(u) => {
		unsafe { $crate::lerfu::Lerfu::new_unchecked('u') }
	};
	(v) => {
		unsafe { $crate::lerfu::Lerfu::new_unchecked('v') }
	};
	(x) => {
		unsafe { $crate::lerfu::Lerfu::new_unchecked('x') }
	};
	(y) => {
		unsafe { $crate::lerfu::Lerfu::new_unchecked('y') }
	};
	(z) => {
		unsafe { $crate::lerfu::Lerfu::new_unchecked('z') }
	};
	('.') => {
		unsafe { $crate::lerfu::Lerfu::new_unchecked('.') }
	};
	(',') => {
		unsafe { $crate::lerfu::Lerfu::new_unchecked(',') }
	};
	('a') => {
		unsafe { $crate::lerfu::Lerfu::new_unchecked('a') }
	};
	('b') => {
		unsafe { $crate::lerfu::Lerfu::new_unchecked('b') }
	};
	('c') => {
		unsafe { $crate::lerfu::Lerfu::new_unchecked('c') }
	};
	('d') => {
		unsafe { $crate::lerfu::Lerfu::new_unchecked('d') }
	};
	('e') => {
		unsafe { $crate::lerfu::Lerfu::new_unchecked('e') }
	};
	('f') => {
		unsafe { $crate::lerfu::Lerfu::new_unchecked('f') }
	};
	('g') => {
		unsafe { $crate::lerfu::Lerfu::new_unchecked('g') }
	};
	('h') => {
		unsafe { $crate::lerfu::Lerfu::new_unchecked('h') }
	};
	('i') => {
		unsafe { $crate::lerfu::Lerfu::new_unchecked('i') }
	};
	('j') => {
		unsafe { $crate::lerfu::Lerfu::new_unchecked('j') }
	};
	('k') => {
		unsafe { $crate::lerfu::Lerfu::new_unchecked('k') }
	};
	('l') => {
		unsafe { $crate::lerfu::Lerfu::new_unchecked('l') }
	};
	('n') => {
		unsafe { $crate::lerfu::Lerfu::new_unchecked('n') }
	};
	('o') => {
		unsafe { $crate::lerfu::Lerfu::new_unchecked('o') }
	};
	('p') => {
		unsafe { $crate::lerfu::Lerfu::new_unchecked('p') }
	};
	('r') => {
		unsafe { $crate::lerfu::Lerfu::new_unchecked('r') }
	};
	('s') => {
		unsafe { $crate::lerfu::Lerfu::new_unchecked('s') }
	};
	('t') => {
		unsafe { $crate::lerfu::Lerfu::new_unchecked('t') }
	};
	('u') => {
		unsafe { $crate::lerfu::Lerfu::new_unchecked('u') }
	};
	('v') => {
		unsafe { $crate::lerfu::Lerfu::new_unchecked('v') }
	};
	('x') => {
		unsafe { $crate::lerfu::Lerfu::new_unchecked('x') }
	};
	('y') => {
		unsafe { $crate::lerfu::Lerfu::new_unchecked('y') }
	};
	('z') => {
		unsafe { $crate::lerfu::Lerfu::new_unchecked('z') }
	};
}
pub use lerfu;
