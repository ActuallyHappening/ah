pub mod vlalehu {
	//! Representations of Lojban characters with type-level niceties

	use std::borrow::Borrow;

	use stodi::Stodi;

	/// TODO is there an official lujvo for this already?
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
		pub unsafe fn new_unchecked(c: char) -> Self {
			Lerfu(c)
		}

		pub fn new(c: char) -> Option<Self> {
			let ret = unsafe { Lerfu::new_unchecked(c) };
			ret.check_stodi().then_some(ret)
		}

		pub fn char(&self) -> char {
			self.0
		}
	}

	impl std::fmt::Display for Lerfu {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			write!(f, "{}", self.char())
		}
	}

	/// Contains no spaces.
	///
	/// # Invariants
	/// Must be morphologically a valid word.
	pub struct WordLerfuVec(Vec<Lerfu>);

	impl WordLerfuVec {
		pub unsafe fn new_unchecked(inner: Vec<Lerfu>) -> Self {
			Self(inner)
		}

		pub fn as_slice(&self) -> &WordLerfuSlice {
			unsafe { WordLerfuSlice::new_unchecked(self.0.as_slice()) }
		}
	}

	impl Borrow<WordLerfuSlice> for WordLerfuVec {
		fn borrow(&self) -> &WordLerfuSlice {
			self.as_slice()
		}
	}

	/// # Invariants
	/// Must be morphologically a valid word.
	#[repr(transparent)]
	pub struct WordLerfuSlice([Lerfu]);

	impl WordLerfuSlice {
		pub unsafe fn new_unchecked(inner: &[Lerfu]) -> &WordLerfuSlice {
			// TODO: Not use transmute here
			unsafe { std::mem::transmute(inner) }
		}
	}

	impl WordLerfuSlice {
		pub fn as_slice(&self) -> &[Lerfu] {
			&self.0
		}
	}

	impl<'w> ToOwned for WordLerfuSlice {
		type Owned = WordLerfuVec;

		fn to_owned(&self) -> Self::Owned {
			unsafe { WordLerfuVec::new_unchecked(self.as_slice().to_vec()) }
		}
	}

	pub mod classifications {
		pub struct Vowels;

		/// Negation of [Vowels]
		pub struct Consenants;

		pub struct Punctuation;
	}
}

pub mod classifications {
	pub mod cmavo {
		use std::borrow::Borrow;

		use ah_sets::Set;

		use crate::vlalehu::{WordLerfuSlice, WordLerfuVec};

		pub struct Cmavo;

		impl<W> Set<W> for Cmavo
		where
			W: Borrow<WordLerfuSlice>,
		{
			fn contains(&self, item: &W) -> bool {
				let word: &WordLerfuSlice = item.borrow();
				todo!()
			}
		}
	}
}
