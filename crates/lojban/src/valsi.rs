//! ```lojban
//! valsi fi la lojban
//! ```
//! ```glico
//! Words in the Lojban language
//! ```

use ah_tcita::veciksi;

pub struct Valsi<'a>(&'a str);

#[veciksi(lojban = "le brivla", glico = "Brivla (word)")]
pub struct Brivla<'a>(&'a str);
