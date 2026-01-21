use crate::sidbo::SidboTcita;

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("Failed to initially connect: {0}")]
	InitialConnect(surrealdb::Error),
	#[error("Failed to use ns use db: {0}")]
	UseNsDb(surrealdb::Error),
	#[error("Failed to sign in: {0}")]
	Signin(surrealdb::Error),
	#[error("Missing env variables, run source env.nu: {key}")]
	MissingEnv { key: String },

	#[error("Failed to select data from db {id:?}: {err}")]
	SelectFailed {
		id: SidboTcita,
		err: surrealdb::Error,
	},
	#[error("Expected data wasn't found: {id:?}")]
	SelectMissing { id: SidboTcita },
	#[error("Failed to select multiple sidbo tcita with ckaji tcita `{ckaji_tcita}`: {err}")]
	SelectCkaji {
		ckaji_tcita: String,
		err: surrealdb::Error,
	},

	#[error("Failed to convert Sidbo from {id:?}: {err_debug}")]
	SidboConversionFailed { id: SidboTcita, err_debug: String },

	#[error("Failed to deserialize extracted {id} ckaji {ckaji_id}: {err}")]
	ExtractCkajiDeserialize {
		id: SidboTcita,
		ckaji_id: String,
		err: serde_json::Error,
	},
	#[error("Failed to extract {id} ckaji {id}")]
	ExtractCkajiMissing { id: SidboTcita, ckaji_id: String },

	#[error("Failed to serialize ckaji {ckaji_id}: {err}")]
	SerializingCkaji {
		ckaji_id: String,
		err: serde_json::Error,
	},

	#[error("Failed to add {id}: {err}")]
	Add {
		id: SidboTcita,
		err: surrealdb::Error,
	},
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
