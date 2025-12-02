pub mod vlalehu {
	//! Representations of Lojban characters with type-level niceties

	/// Lojban character literal stored in memory as a UTF-8 compatible code point using [char]
	pub struct Lerfu(char);

	impl Lerfu {
		pub const ALLOWED: [char; 26] = [
			'\'', ',', '.', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
			'r', 's', 't', 'u', 'v', 'x', 'y', 'z',
		];
	}
}
