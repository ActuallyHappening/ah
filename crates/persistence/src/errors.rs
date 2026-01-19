pub enum Error {
	InitialConnect(surrealdb::Error),
	UseNsDb(surrealdb::Error),
	Signin(surrealdb::Error),
	MissingEnv { key: String },
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
