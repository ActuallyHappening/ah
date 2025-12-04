pub trait Set<El> {
	fn contains(&self, item: &El) -> bool;
}

pub trait FiniteSet<El>: Set<El> {
	fn all_in_memory(&self) -> &[El];

	/// TODO LOJBAN: cardinality of set
	fn cardinality(&self) -> usize {
		self.all_in_memory().len()
	}
}

pub mod identities;
