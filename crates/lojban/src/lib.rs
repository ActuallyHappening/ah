pub mod tcita {
	//! tcita = x1 name/label/tag of x2

	use crate::vlalehu::{TiValsiLaLojban_Slice, TiValsiLaLojban_Vec};

	pub trait Tcita_ti {
		/// TODO LOJBAN: *Full* name of self as an *abstraction* concept,
		/// implying uniqueness
		fn full_abstract_tcita_ti() -> &'static TiValsiLaLojban_Slice;
	}
}
pub mod vlalehu;

pub mod classifications {
	pub mod cmavo {
		use std::borrow::Borrow;

		use ah_sets::Set;

		use crate::vlalehu::{TiValsiLaLojban_Slice, TiValsiLaLojban_Vec};

		pub struct Cmavo;

		impl<W> Set<W> for Cmavo
		where
			W: Borrow<TiValsiLaLojban_Slice>,
		{
			fn contains(&self, item: &W) -> bool {
				let word: &TiValsiLaLojban_Slice = item.borrow();
				todo!()
			}
		}
	}
}
