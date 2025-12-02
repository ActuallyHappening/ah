pub mod vlalehu {
	//! Representations of Lojban characters with type-level niceties

	use stodi::Stodi;

	/// TODO is there an official lujvo for this already?
	///
	/// Lojban character literal stored in memory as a UTF-8 compatible code point using [char]
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
	}
}
