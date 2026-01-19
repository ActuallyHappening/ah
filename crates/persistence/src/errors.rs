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
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
