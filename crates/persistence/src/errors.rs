use crate::sidbo::SidboTcita;

pub enum Error {
	InitialConnect(surrealdb::Error),
	UseNsDb(surrealdb::Error),
	Signin(surrealdb::Error),
	MissingEnv {
		key: String,
	},
	SelectFailed {
		id: SidboTcita,
		err: surrealdb::Error,
	},
	SelectMissing {
		id: SidboTcita,
	},
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
