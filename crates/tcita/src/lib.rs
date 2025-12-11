//! ```lojban
//! si'o tcita gau la lojban
//! ```
//! ```glico
//! Label in language lojban
//! ```

pub use ah_tcita_macros::*;

/// ```lojban
/// ka tcita
/// ```
/// ```glico
/// The property of being|having a label
/// ```
pub trait Ka_tcita {
	/// ```lojban
	/// ti seltcita bau la lojban
	/// .idu ti se tcita bau la lojban
	/// ```
	/// ```glico
	/// This thing's label in Lojban
	/// ```
	const TI_SELTCITA_BAU_LA_LOJBAN: &str;
	/// [Self::TI_SELTCITA_BAU_LA_LOJBAN]
	const TCITA: &str = Self::TI_SELTCITA_BAU_LA_LOJBAN;
}
