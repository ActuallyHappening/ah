//! ```lojban
//! valsi fi la lojban
//! ```
//! ```glico
//! Words in the Lojban language
//! ```

use ah_tcita::veciksi;
use stodi::Stodi;

use crate::lerfu::Le_jbolehu;

pub struct Valsi<'a>(&'a str);

#[veciksi(lojban = "le brivla", glico = "Brivla (word)")]
pub struct Brivla<'a>(&'a str);

pub struct Cmavo<'a>(&'a str);

#[veciksi(lojban = "le cmavo (la .FA.)", glico = "FA cmavo class")]
pub struct Cmavo_FA<'a>(&'a str);

impl Cmavo_FA<'_> {
	pub const ALLOWED: [&'static str; 6] = ["fa", "fe", "fi", "fo", "fu", "fi'a"];
}

impl Stodi for Cmavo_FA<'_> {
	fn check_stodi(&self) -> bool {
		Self::ALLOWED.contains(&self.0)
	}
}
