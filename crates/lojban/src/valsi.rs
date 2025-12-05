//! Things to note:
//! For understanding, by [char] processing is employed even though this is
//! inefficient

use std::{borrow::Borrow, ops::Deref, sync::Arc};

use crate::lerfu::Lerfu;

/// Contains no spaces.
///
/// # Invariants
/// See [TiValsiLaLojban_Slice]
#[derive(Debug, Clone)]
pub struct TiValsiLaLojban_String(String);
pub type ValsiString = TiValsiLaLojban_String;

impl Stodi for TiValsiLaLojban_String {
	fn check_stodi(&self) -> bool {
		self.as_slice().check_stodi()
	}
}

/// Readonly methods implemented on [TiValsiLaLojban_Slice] instead
impl Deref for TiValsiLaLojban_String {
	type Target = TiValsiLaLojban_Str;

	fn deref(&self) -> &Self::Target {
		self.as_slice()
	}
}

/// Creation paths, not parsing
impl TiValsiLaLojban_String {
	pub fn new(inner: String) -> Option<TiValsiLaLojban_String> {
		let ret = unsafe { TiValsiLaLojban_String::new_unchecked(inner) };
		ret.check_stodi().then_some(ret)
	}
}

/// Implementation understanding methods.
/// Invariant cautious
impl TiValsiLaLojban_String {
	pub unsafe fn new_unchecked(inner: String) -> Self {
		Self(inner)
	}

	pub fn as_slice(&self) -> &TiValsiLaLojban_Str {
		unsafe { TiValsiLaLojban_Str::new_unchecked(self.0.as_bytes()) }
	}
}

impl Borrow<TiValsiLaLojban_Str> for TiValsiLaLojban_String {
	fn borrow(&self) -> &TiValsiLaLojban_Str {
		&self
	}
}

/// [ti] Word[valsi] [la] in Lojban[lojban]
/// # Invariants
/// Must be morphologically a valid la lojban word.
#[derive(Debug)]
#[repr(transparent)]
pub struct TiValsiLaLojban_Str(str);
pub type ValsiStr = TiValsiLaLojban_Str;

impl Stodi for TiValsiLaLojban_Str {
	fn check_stodi(&self) -> bool {
		let lerfu = match self.checked_iter() {
			Some(lerfu) => lerfu,
			None => return false,
		};
		// todo!()
		true
	}
}

/// Invariant cautious
impl TiValsiLaLojban_Str {
	/// Cautious around invariant
	fn checked_iter(&self) -> Option<impl IntoIterator<Item = Lerfu>> {
		self
			.0
			.chars()
			.map(|c| Lerfu::new(c))
			.collect::<Option<Vec<_>>>()
	}
}

/// Implementation understanding methods,
/// each must uphold invariant always
impl TiValsiLaLojban_Str {
	#[ensures(inner == &ret.0)]
	pub unsafe fn new_unchecked(inner: &str) -> &TiValsiLaLojban_Str {
		// TODO: Not use transmute here
		unsafe { &*(inner as *const str as *const TiValsiLaLojban_Str) }
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

impl<'w> ToOwned for TiValsiLaLojban_Str {
	type Owned = TiValsiLaLojban_String;

	fn to_owned(&self) -> Self::Owned {
		unsafe { TiValsiLaLojban_String::new_unchecked(String::from_utf8_unchecked(self.0.to_vec())) }
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

use contracts::ensures;
use stodi::{Stodi, invariant};
pub use valsi;
