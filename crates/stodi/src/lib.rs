/// For invariants that are cheap to check
pub trait Stodi {
	fn check_stodi(&self) -> bool;

	/// TODO LOJBAN: Name this better
	fn stodi_debug_msg(&self) -> std::borrow::Cow<'_, str> {
		std::borrow::Cow::Borrowed("Stodi[Invariant] violated")
	}
}

/// TODO make a new stodi-macros crate which adds this attribute
pub use contracts::invariant;
