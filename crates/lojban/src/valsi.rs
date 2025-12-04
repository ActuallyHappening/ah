use std::{borrow::Borrow, ops::Deref};

use crate::lerfu::Lerfu;

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

impl Deref for TiValsiLaLojban_Vec {
	type Target = TiValsiLaLojban_Slice;

	fn deref(&self) -> &Self::Target {
		self.as_slice()
	}
}

impl Borrow<TiValsiLaLojban_Slice> for TiValsiLaLojban_Vec {
	fn borrow(&self) -> &TiValsiLaLojban_Slice {
		&self
	}
}

/// [ti] Word[valsi] [la] in Lojban[lojban]
/// # Invariants
/// Must be morphologically a valid word.
/// Therefore, can't be empty
#[repr(transparent)]
pub struct TiValsiLaLojban_Slice([Lerfu]);

impl Stodi for TiValsiLaLojban_Slice {
	fn check_stodi(&self) -> bool {
		// todo!()
		true
	}
}

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

#[test]
fn test_valsi_macro() {
	// valsi!(abc);
}

#[macro_export]
macro_rules! valsi {
	($($letter:ident),+) => {
		[
			$($crate::lerfu::lerfu!($letter)),+
		]
	};
}

use stodi::Stodi;
pub use valsi;
