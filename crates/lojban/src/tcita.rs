//! tcita = x1 name/label/tag of x2

use crate::{
	suhore_valsi::SuhoreValsi_Slice,
	valsi::{TiValsiLaLojban_Str, TiValsiLaLojban_String},
};

pub trait Tcita_ti {
	/// TODO LOJBAN: *Full* name of self as an *abstraction* concept,
	/// implying uniqueness
	///
	/// # Invariant
	/// Is valid Lojban
	/// TODO ABSTRACTION: What constructions are valid as names?
	fn full_abstract_tcita_ti() -> &'static str;
}
