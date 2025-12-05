//! TODO LOJBAN: Makes sure the name is grammatically correct
//!
//! More than one [su'ore] word [valsi]
//!
//! # Invariants
//! Must be a valid Lojban sentence (at all times).

use stodi::Stodi;

use crate::valsi::{TiValsiLaLojban_Str, TiValsiLaLojban_String, ValsiStr, ValsiString};

// #[derive(Debug, Clone)]
// pub struct SuhoreValsi_String(Arc<String>);

// impl Stodi for SuhoreValsi_String {
// 	fn check_stodi(&self) -> bool {
// 		// self.
// 	}
// }

// /// Creation paths
// impl SuhoreValsi_String {
// 	pub fn new(inner: impl IntoIterator<Item = TiValsiLaLojban_String>) -> Option<Self> {
// 		let ret = unsafe { Self::new_unchecked(inner.into_iter().collect::<Vec<_>>()) };
// 		ret.check_stodi().then_some(ret)
// 	}
// }

// /// Implementation understanding methods.
// /// Invariant cautious
// impl SuhoreValsi_String {
// 	pub const unsafe fn new_unchecked(inner: Vec<TiValsiLaLojban_String>) -> Self {
// 		SuhoreValsi_String(inner)
// 	}

// 	pub fn as_slice(&self) -> &[&ValsiSlice] {
// 		self
// 			.0
// 			.as_slice()
// 			.map(|valsi_string| valsi_string.as_slice())
// 	}
// }

#[derive(Debug)]
#[repr(transparent)]
pub struct SuhoreValsi_Slice(str);

impl Stodi for SuhoreValsi_Slice {
	fn check_stodi(&self) -> bool {
		if self.0.len() < 2 {
			return false;
		}
		// TODO SOUNDNESS: actually parse
		true
	}
}

impl SuhoreValsi_Slice {
	pub unsafe fn new_unchecked(inner: &str) -> &Self {
		unsafe { &*(inner as *const str as *const SuhoreValsi_Slice) }
	}
}
