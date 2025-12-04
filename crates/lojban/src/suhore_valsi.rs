//! TODO LOJBAN: Makes sure the name is grammatically correct
//!
//! More than one [su'ore] word [valsi]
//!
//! # Invariants
//! Must be a valid Lojban sentence (at all times).

use stodi::Stodi;

use crate::valsi::{TiValsiLaLojban_Slice, TiValsiLaLojban_Vec};

pub struct SuhoreValsi_Vec(Vec<TiValsiLaLojban_Vec>);

impl SuhoreValsi_Vec {
	pub const unsafe fn new_unchecked(inner: Vec<TiValsiLaLojban_Vec>) -> Self {
		SuhoreValsi_Vec(inner)
	}
}

impl Stodi for SuhoreValsi_Vec {
	fn check_stodi(&self) -> bool {
		self.0.len() >= 2
	}
}

pub struct SuhoreValsi_Slice<'valsi>([&'valsi TiValsiLaLojban_Slice]);
