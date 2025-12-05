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
	/// TODO: Add the invariant to the type signature
	fn tcita_ti_full_abstract() -> &'static str;

	/// TODO: Experiment with ID including apostrophe's[su'o y'y] in id's instead of 'h's
	fn tcita_index_url() -> String {
		fn url_encode(s: &str) -> String {
			s.replace("'", "h").replace(" ", "%20")
		}
		format!(
			"https://github.com/ActuallyHappening/ah/blob/master/index.md#{}",
			url_encode(Self::tcita_ti_full_abstract())
		)
	}
}
