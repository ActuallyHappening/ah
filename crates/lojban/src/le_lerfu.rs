use ah_sets::Set;
use stodi::Stodi;

use crate::siho_selylehu::Sohi_lojbo_selylehu;

/// ```lojban
/// jbole'u
/// .idu lojbo lerfu
/// ```
/// ```english
/// Lojban character
/// ```
#[derive(Clone, Copy)]
pub struct Le_jbolehu(u8);

impl Stodi for Le_jbolehu {
	fn check_stodi(&self) -> bool {
		Sohi_lojbo_selylehu::default().contains(&self.0)
	}

	fn check_ro_stodi(&self) -> bool {
		self.check_stodi() && format!("{}", self) == self.char().to_string()
	}
}

impl Le_jbolehu {
	#[contracts::requires(self.check_stodi())]
	pub fn char(self) -> char {
		char::try_from(self.0).unwrap()
	}
}

impl std::fmt::Display for Le_jbolehu {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.char())
	}
}

impl std::fmt::Debug for Le_jbolehu {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_tuple("Jbole'u").field(&self.0).finish()
	}
}
