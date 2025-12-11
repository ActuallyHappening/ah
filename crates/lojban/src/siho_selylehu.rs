//! ```lojban
//! si'o selyle'u
//! .idu si'o se lerfu
//! ```
//! ```english
//! The concepts|ideas about alphabet/s
//! ```

use ah_sets::Set;
use stodi::Stodi;

/// ```lojban
/// si'o selyle'u
/// ```
/// ```english
/// The concept|idea of an alphabet.
/// The trait for a character set.
/// ```
/// The `Set<u8>` should be an upcast from `u8` to `char`
pub trait Sohi_selylehu: Set<char> + Set<u8> {}

/// ```lojban
/// so'i lojbo selyle'u
/// ```
/// ```english
/// The concept of (specifically) the Lojban alphabet
/// ```
#[derive(Clone, Copy, Default)]
pub struct Sohi_lojbo_selylehu;

impl Sohi_lojbo_selylehu {
	pub const ALLOWED_REF: &[u8] = b"',.abcdefgijklmnoprstuvxyz";
	pub const ALLOWED: [u8; 26] = [
		39, 44, 46, 97, 98, 99, 100, 101, 102, 103, 105, 106, 107, 108, 109, 110, 111, 112, 114, 115,
		116, 117, 118, 120, 121, 122,
	];
}

impl Stodi for Sohi_lojbo_selylehu {
	fn check_stodi(&self) -> bool {
		&Self::ALLOWED == Self::ALLOWED_REF
	}
}

impl Set<u8> for Sohi_lojbo_selylehu {
	fn contains(&self, item: &u8) -> bool {
		Self::ALLOWED.contains(item)
	}
}

impl Set<char> for Sohi_lojbo_selylehu {
	fn contains(&self, item: &char) -> bool {
		let Ok(u8) = u8::try_from(*item) else {
			return false;
		};
		Self::ALLOWED.contains(&u8)
	}
}

#[test]
fn stodi_muha_sohi_lojbo_selylehu() {
	assert!(Sohi_lojbo_selylehu::default().check_stodi());
}
