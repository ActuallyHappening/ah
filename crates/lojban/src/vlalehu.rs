//! Representations of Lojban characters with type-level niceties

use std::borrow::Borrow;

use stodi::Stodi;

pub mod lerfu {

	/// TODO LOJBAN is there an official lujvo for this already?
	///
	/// Lojban character literal stored in memory as a UTF-8 compatible code point using [char]
	#[derive(Clone, Copy)]
	pub struct Lerfu(char);

	impl Lerfu {
		pub const ALLOWED: [char; 26] = [
			'\'', ',', '.', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
			'r', 's', 't', 'u', 'v', 'x', 'y', 'z',
		];
	}

	impl stodi::Stodi for Lerfu {
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
}

pub mod valsi {
	use std::borrow::Borrow;

	use crate::vlalehu::lerfu::Lerfu;

	/// Contains no spaces.
	///
	/// # Invariants
	/// Must be morphologically a valid word.
	pub struct TiValsiLaLojban_Vec(Vec<Lerfu>);

	impl TiValsiLaLojban_Vec {
		pub unsafe fn new_unchecked(inner: Vec<Lerfu>) -> Self {
			Self(inner)
		}

		pub fn as_slice(&self) -> &TiValsiLaLojban_Slice {
			unsafe { TiValsiLaLojban_Slice::new_unchecked(self.0.as_slice()) }
		}
	}

	impl Borrow<TiValsiLaLojban_Slice> for TiValsiLaLojban_Vec {
		fn borrow(&self) -> &TiValsiLaLojban_Slice {
			self.as_slice()
		}
	}

	/// [ti] Word[valsi] [la] in Lojban[lojban]
	/// # Invariants
	/// Must be morphologically a valid word.
	#[repr(transparent)]
	pub struct TiValsiLaLojban_Slice([Lerfu]);

	impl TiValsiLaLojban_Slice {
		pub unsafe fn new_unchecked(inner: &[Lerfu]) -> &TiValsiLaLojban_Slice {
			// TODO: Not use transmute here
			unsafe { std::mem::transmute(inner) }
		}
	}

	impl TiValsiLaLojban_Slice {
		pub fn as_slice(&self) -> &[Lerfu] {
			&self.0
		}
	}

	impl<'w> ToOwned for TiValsiLaLojban_Slice {
		type Owned = TiValsiLaLojban_Vec;

		fn to_owned(&self) -> Self::Owned {
			unsafe { TiValsiLaLojban_Vec::new_unchecked(self.as_slice().to_vec()) }
		}
	}
}

pub mod classifications {
	use ah_sets::{FiniteSet, Set};

	use crate::{tcita::Tcita_ti, vlalehu::Lerfu};

	/// Set[lo'i] of single[pa]-letter[lerfu] vowels[karsna]
	pub struct Lohi_pa_lerfu_karsna;

	impl Lohi_pa_lerfu_karsna {
		pub const PA_LERFU: [Lerfu; 6] = [
			Lerfu('a'),
			Lerfu('e'),
			Lerfu('i'),
			Lerfu('o'),
			Lerfu('u'),
			Lerfu('y'),
		];
	}

	impl Set<Lerfu> for Lohi_pa_lerfu_karsna {
		fn contains(&self, item: &Lerfu) -> bool {
			Self::PA_LERFU.contains(item)
		}
	}

	impl FiniteSet<Lerfu> for Lohi_pa_lerfu_karsna {
		fn all_in_memory(&self) -> &[Lerfu] {
			&Self::PA_LERFU
		}
	}

	pub struct Punctuation;
}
