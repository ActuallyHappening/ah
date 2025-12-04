//! Things to note:
//! For understanding, by [char] processing is employed even though this is
//! inefficient

use std::{borrow::Borrow, ops::Deref};

use crate::lerfu::Lerfu;

/// Contains no spaces.
///
/// # Invariants
/// See [TiValsiLaLojban_Slice]
pub struct TiValsiLaLojban_Vec(Vec<Lerfu>);

impl Stodi for TiValsiLaLojban_Vec {
	fn check_stodi(&self) -> bool {
		self.as_slice().check_stodi()
	}
}

/// Readonly methods implemented on [TiValsiLaLojban_Slice] instead
impl Deref for TiValsiLaLojban_Vec {
	type Target = TiValsiLaLojban_Slice;

	fn deref(&self) -> &Self::Target {
		self.as_slice()
	}
}

/// Creation paths, not parsing
impl TiValsiLaLojban_Vec {
	pub fn new(inner: impl IntoIterator<Item = Lerfu>) -> Option<TiValsiLaLojban_Vec> {
		let ret = unsafe { TiValsiLaLojban_Vec::new_unchecked(inner.into_iter().collect()) };
		ret.check_stodi().then_some(ret)
	}

	pub fn new_from_str(s: impl Borrow<str>) -> Option<TiValsiLaLojban_Vec> {
		let sohu_lerfu = s
			.borrow()
			.chars()
			.map(|c| Lerfu::new(c))
			.collect::<Option<Vec<_>>>()?;
		Self::new(sohu_lerfu)
	}
}

/// Implementation understanding methods
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

/// Creation paths
impl TiValsiLaLojban_Slice {
	pub fn new(sohu_lerfu: &[Lerfu]) -> Option<&TiValsiLaLojban_Slice> {
		let ret = unsafe { TiValsiLaLojban_Slice::new_unchecked(sohu_lerfu) };
		ret.check_stodi().then_some(ret)
	}
}

/// Implementation understanding methods
impl TiValsiLaLojban_Slice {
	pub unsafe fn new_unchecked(inner: &[Lerfu]) -> &TiValsiLaLojban_Slice {
		// TODO: Not use transmute here
		unsafe { std::mem::transmute(inner) }
	}

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
