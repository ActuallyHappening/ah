//! Things to note:
//! For understanding, by [char] processing is employed even though this is
//! inefficient

use std::{borrow::Borrow, ops::Deref};

use crate::lerfu::Lerfu;

/// Contains no spaces.
///
/// # Invariants
/// See [TiValsiLaLojban_Slice]
#[derive(Debug, Clone)]
pub struct TiValsiLaLojban_Vec(String);
pub type ValsiVec = TiValsiLaLojban_Vec;

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
	pub fn new(inner: String) -> Option<TiValsiLaLojban_Vec> {
		let ret = unsafe { TiValsiLaLojban_Vec::new_unchecked(inner) };
		ret.check_stodi().then_some(ret)
	}
}

/// Implementation understanding methods
impl TiValsiLaLojban_Vec {
	pub unsafe fn new_unchecked(inner: String) -> Self {
		Self(inner)
	}

	pub fn as_slice(&self) -> &TiValsiLaLojban_Slice {
		unsafe { TiValsiLaLojban_Slice::new_unchecked(self.0.as_bytes()) }
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
pub struct TiValsiLaLojban_Slice([u8]);
pub type ValsiSlice = TiValsiLaLojban_Slice;

impl Stodi for TiValsiLaLojban_Slice {
	fn check_stodi(&self) -> bool {
		let lerfu = match self.checked_iter() {
			Some(lerfu) => lerfu,
			None => return false,
		};
		// todo!()
		true
	}
}

/// Creation paths
impl TiValsiLaLojban_Slice {
	pub fn new(sohu_lerfu: &str) -> Option<&TiValsiLaLojban_Slice> {
		let ret = unsafe { TiValsiLaLojban_Slice::new_unchecked(sohu_lerfu.as_bytes()) };
		ret.check_stodi().then_some(ret)
	}
}

/// Invariant cautious 
impl TiValsiLaLojban_Slice {
	
}

/// Implementation understanding methods,
/// each must uphold invariant always
impl TiValsiLaLojban_Slice {
	#[invariant(inner == &ret.0)]
	pub unsafe fn new_unchecked(inner: &[u8]) -> &TiValsiLaLojban_Slice {
		// TODO: Not use transmute here
		unsafe { std::mem::transmute::<&[u8], &TiValsiLaLojban_Slice>(inner) }
	}

	/// Catious around invariant
	fn checked_as_str(&self) -> Option<&str> {
		std::str::from_utf8(&self.0).ok()
	}

	/// Catious around invariant
	fn checked_iter(&self) -> Option<impl Iterator<Item = Lerfu>> {
		self
			.checked_as_str()
			.map(|str| str.chars().map(|c| Lerfu::new(c).unwrap()))
	}

	#[stodi::invariant(self.check_stodi())]
	pub fn as_str(&self) -> &str {
		self.checked_as_str().unwrap()
	}

	#[stodi::invariant(self.check_stodi())]
	pub fn iter(&self) -> impl Iterator<Item = Lerfu> {
		self.checked_iter().unwrap()
	}
}

impl<'w> ToOwned for TiValsiLaLojban_Slice {
	type Owned = TiValsiLaLojban_Vec;

	fn to_owned(&self) -> Self::Owned {
		unsafe { TiValsiLaLojban_Vec::new_unchecked(String::from_utf8_unchecked(self.0.to_vec())) }
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

use stodi::{Stodi, invariant};
pub use valsi;
