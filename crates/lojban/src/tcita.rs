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

	fn tcita_index_url() -> String {
		/// TODO: Experiment with ID including apostrophe's[su'o y'y] in id's instead of 'h's
		fn github_url_encode(s: &str) -> String {
			s.replace("'", "").replace(".", "").replace(" ", "-")
		}
		format!(
			"https://github.com/ActuallyHappening/ah/blob/master/index.md#{}",
			github_url_encode(Self::tcita_ti_full_abstract())
		)
	}
}

#[cfg(test)]
mod tests {
	use crate::lerfu::CharNotLerfu;

	use super::*;

	struct Test;

	impl Tcita_ti for Test {
		fn tcita_ti_full_abstract() -> &'static str {
			"le .cipra. .i y'y"
		}
	}

	#[test]
	fn test_tcita_cipra_index_url() {
		let url = Test::tcita_index_url();
		let expected = "https://github.com/ActuallyHappening/ah/blob/master/index.md#le-cipra-i-yy";
		dbg!(&url);
		assert_eq!(url, expected)
	}

	#[test]
	fn test_tcita_lerfu_exception_index_url() {
		let url = CharNotLerfu::tcita_index_url();
		let expected =
			"https://github.com/ActuallyHappening/ah/blob/master/index.md#ti-daavni-le-parsing-lerfu";
		dbg!(&url);
		assert_eq!(url, expected)
	}
}
