use crate::Set;

#[derive(Debug, Default)]
pub struct NullSet;

impl<I> Set<I> for NullSet {
	fn contains(&self, _item: &I) -> bool {
		false
	}
}

impl NullSet {
	pub fn new() -> Self {
		Self
	}
}

/// Set with only one valid element
#[derive(Debug)]
pub struct Singleton<I> {
	pub value: I,
}

impl<I: PartialEq> Set<I> for Singleton<I> {
	fn contains(&self, item: &I) -> bool {
		&self.value == item
	}
}

impl<I> Singleton<I> {
	pub fn new(value: I) -> Self {
		Self { value }
	}
}
