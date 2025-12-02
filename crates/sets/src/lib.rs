pub trait Set<Item> {
	fn contains(&self, item: &Item) -> bool;
}

pub mod identities;
