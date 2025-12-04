pub mod tcita {
	//! tcita = x1 name/label/tag of x2

	use crate::valsi::{TiValsiLaLojban_Slice, TiValsiLaLojban_Vec};

	pub trait Tcita_ti {
		/// TODO LOJBAN: *Full* name of self as an *abstraction* concept,
		/// implying uniqueness
		fn full_abstract_tcita_ti() -> &'static TiValsiLaLojban_Slice;
	}
}
pub mod lerfu;
pub mod valsi;
// pub mod classifications {
// 	use ah_sets::{FiniteSet, Set};

// 	use crate::{tcita::Tcita_ti, vlalehu::lerfu::Lerfu};

// 	/// Set[lo'i] of single[pa]-letter[lerfu] vowels[karsna]
// 	pub struct Lohi_pa_lerfu_karsna;

// 	impl Lohi_pa_lerfu_karsna {
// 		pub const PA_LERFU: [Lerfu; 6] = [
// 			Lerfu('a'),
// 			Lerfu('e'),
// 			Lerfu('i'),
// 			Lerfu('o'),
// 			Lerfu('u'),
// 			Lerfu('y'),
// 		];
// 	}

// 	impl Set<Lerfu> for Lohi_pa_lerfu_karsna {
// 		fn contains(&self, item: &Lerfu) -> bool {
// 			Self::PA_LERFU.contains(item)
// 		}
// 	}

// 	impl FiniteSet<Lerfu> for Lohi_pa_lerfu_karsna {
// 		fn all_in_memory(&self) -> &[Lerfu] {
// 			&Self::PA_LERFU
// 		}
// 	}

// 	pub struct Punctuation;
// }

// pub mod classifications {
// 	pub mod cmavo {
// 		use std::borrow::Borrow;

// 		use ah_sets::Set;

// 		use crate::vlalehu::valsi::{TiValsiLaLojban_Slice, TiValsiLaLojban_Vec};

// 		pub struct Cmavo;

// 		impl<W> Set<W> for Cmavo
// 		where
// 			W: Borrow<TiValsiLaLojban_Slice>,
// 		{
// 			fn contains(&self, item: &W) -> bool {
// 				let word: &TiValsiLaLojban_Slice = item.borrow();
// 				todo!()
// 			}
// 		}
// 	}
// }
