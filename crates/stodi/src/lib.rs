/// For invariants that are cheap to check
pub trait Stodi {
	/// Should be cheap to check, runtime at most O(n) as self grows
	fn check_stodi(&self) -> bool;

	/// Can be much slower, for documentation purposes.
	/// Must call [Stodi::check_stodi] first, or repeat identical logic.
	/// If [Stodi::check_stodi] returns `false`, this must too.
	fn check_ro_stodi(&self) -> bool {
		self.check_stodi()
	}

	/// TODO LOJBAN: Name this better
	fn stodi_debug_msg(&self) -> std::borrow::Cow<'_, str> {
		std::borrow::Cow::Borrowed("Stodi[Invariant] violated")
	}
}

/// TODO make a new stodi-macros crate which adds this attribute
pub use contracts::invariant;
