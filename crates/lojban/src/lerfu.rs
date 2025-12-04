use stodi::Stodi;

/// Full name: pa lerfu la lojban
/// TODO LOJBAN is there an official lujvo for this already?
///
/// Lojban character literal stored in memory as a UTF-8 compatible code point using [char]
#[derive(Clone, Copy, Debug)]
pub struct Lerfu(char);

impl Tcita_ti for Lerfu {
	fn full_abstract_tcita_ti() -> &'static SuhoreValsi_Slice<'static> {
		todo!()
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
	pub const unsafe fn new_unchecked(c: char) -> Self {
		Lerfu(c)
	}

	pub fn new(c: char) -> Option<Self> {
		let ret = unsafe { Lerfu::new_unchecked(c) };
		ret.check_stodi().then_some(ret)
	}

	pub const fn char(&self) -> char {
		self.0
	}

	pub unsafe fn unchecked_from_array<const N: usize>(arr: [char; N]) -> [Lerfu; N] {
		arr.map(|char| unsafe { Lerfu::new_unchecked(char) })
	}
}

impl std::fmt::Display for Lerfu {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.char())
	}
}

impl PartialEq for Lerfu {
	fn eq(&self, other: &Self) -> bool {
		// TODO LOJBAN: Word this context switch well
		assert!(self.check_stodi(), "PartialEq: {}", self.stodi_debug_msg());
		assert!(
			other.check_stodi(),
			"PartialEq: {}",
			other.stodi_debug_msg()
		);
		self.0.eq_ignore_ascii_case(&other.0)
	}
}

const a: Lerfu = lerfu!(h);

#[test]
fn lerfu_macro() {
	assert_eq!(lerfu!(h), lerfu!('\''));
	assert_eq!(lerfu!(h), Lerfu::new('\'').unwrap());

	assert_eq!(lerfu!(a), Lerfu::new('a').unwrap());
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
}
pub use lerfu;

use crate::{suhore_valsi::SuhoreValsi_Slice, tcita::Tcita_ti};
