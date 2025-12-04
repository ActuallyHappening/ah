//! TODO LOJBAN: Makes sure the name is grammatically correct
//!
//! More than one [su'ore] word [valsi]
//!
//! # Invariants
//! Must be a valid Lojban sentence (at all times).

use stodi::Stodi;

use crate::valsi::{TiValsiLaLojban_Slice, TiValsiLaLojban_Vec};

pub struct SuhoreValsi_Vec(Vec<TiValsiLaLojban_Vec>);

impl Stodi for SuhoreValsi_Vec {
	fn check_stodi(&self) -> bool {
		if self.0.len() <= 1 {
			return false;
		}
		// TODO SOUNDNESS
		true
	}
}

impl SuhoreValsi_Vec {
	pub fn new(inner: impl IntoIterator<Item = TiValsiLaLojban_Vec>) -> Option<Self> {
		let ret = unsafe { Self::new_unchecked(inner.into_iter().collect::<Vec<_>>()) };
		ret.check_stodi().then_some(ret)
	}
}

/// Implementation understanding methods
impl SuhoreValsi_Vec {
	pub const unsafe fn new_unchecked(inner: Vec<TiValsiLaLojban_Vec>) -> Self {
		SuhoreValsi_Vec(inner)
	}
}

pub struct SuhoreValsi_Slice<'valsi>([&'valsi TiValsiLaLojban_Slice]);

impl Stodi for SuhoreValsi_Slice<'_> {
	fn check_stodi(&self) -> bool {
		self.0.len() >= 2
	}
}

impl<'valsi> SuhoreValsi_Slice<'valsi> {
	pub unsafe fn new_unchecked<'s>(inner: &'s [&'valsi TiValsiLaLojban_Slice]) -> &'s Self {
		// TODO: Don't use transmute here!
		unsafe { std::mem::transmute(inner) }
	}
}
