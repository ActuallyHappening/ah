use std::{collections::HashMap, hash::Hash};

use ah_tcita::Ka_tcita;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::prelude::*;

/// [PartialEq] .e [Eq] use only the [Sidbo.id] for identity
#[derive(Debug, Deserialize, Eq)]
pub struct Sidbo {
	pub(crate) id: SidboTcita,
	pub(crate) ckaji: HashMap<String, serde_json::Value>,
}

impl PartialEq for Sidbo {
	fn eq(&self, other: &Self) -> bool {
		self.id.eq(&other.id)
	}
}

impl Hash for Sidbo {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.id.hash(state);
	}
}

impl Sidbo {
	pub fn id(self) -> SidboTcita {
		self.id
	}

	pub fn get_id(&self) -> &SidboTcita {
		&self.id
	}

	pub fn extract_ckaji<Ckaji>(&mut self) -> Result<Ckaji>
	where
		Ckaji: DeserializeOwned + Ka_tcita,
	{
		let id = Ckaji::TCITA;
		if let Some(entry) = self.ckaji.remove(id) {
			Ok(
				serde_json::from_value(entry).map_err(|err| Error::ExtractCkajiDeserialize {
					id: self.id.clone(),
					ckaji_id: id.to_owned(),
					err,
				})?,
			)
		} else {
			Err(Error::ExtractCkajiMissing {
				id: self.id.clone(),
				ckaji_id: id.to_owned(),
			})
		}
	}
}

#[veciksi(
	lojban = "TODO le packaji sidbo",
	glico = "A generic sidbo with only pa (one) ckaji (property) for type convenience"
)]
#[macro_export]
macro_rules! packaji_sidbo {
	($vis:vis struct $ident:ident { ckaji: $ckaji:ty, ... }) => {
		#[derive(Debug, Deserialize)]
		#[serde(try_from = "Sidbo")]
		$vis struct $ident {
			ckaji: $ckaji,
			id: SidboTcita,
		}

		impl TryFrom<Sidbo> for $ident
		where
			$ckaji: DeserializeOwned + Ka_tcita,
		{
			type Error = Error;

			fn try_from(mut sidbo: Sidbo) -> Result<Self, Self::Error> {
				let ckaji = sidbo.extract_ckaji::<$ckaji>()?;
				let id = sidbo.id();
				Ok(Self { id, ckaji })
			}
		}
	};
}

pub use packaji_sidbo;

/// Don't optimize for space. Optimize for simplicity.
/// We could use just RecordIdKey here, but then deserialization is harder for no significant reason
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct SidboTcita(pub(crate) surrealdb::RecordId);

impl SidboTcita {
	pub(crate) const TB: &str = "sidbo";
}

impl SidboTcita {
	pub fn from_tcita<T>() -> Self
	where
		T: Ka_tcita,
	{
		SidboTcita(surrealdb::RecordId::from_table_key(Self::TB, T::TCITA))
	}

	pub fn from_name(name: &str) -> Self {
		SidboTcita(surrealdb::RecordId::from_table_key(Self::TB, name))
	}

	pub(crate) fn raw(&self) -> &surrealdb::RecordId {
		&self.0
	}
}

// impl<R> IntoResource<Option<R>> for &SidboTcita {
// 	fn into_resource(self) -> Result<surrealdb::opt::Resource, surrealdb::Error> {
// 		#[allow(deprecated)]
// 		IntoResource::<Option<R>>::into_resource(self.0.clone())
// 	}
// }

impl std::fmt::Display for SidboTcita {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}
