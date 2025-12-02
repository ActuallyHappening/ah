/// For invariants that are cheap to check
pub trait Stodi {
	fn check_invariant(&self) -> bool;
}

/// TODO make a new stodi-macros crate which adds this attribute
pub use contracts::invariant;
