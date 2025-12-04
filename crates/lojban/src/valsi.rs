use std::borrow::Borrow;

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
