use crate::prelude::*;

pub struct PersistenceEngineBuilder {
	ns: String,
	db: String,
	url: Url,
	user: String,
	pass: String,
}

pub struct PersistenceEngine {
	conn: surrealdb::Surreal<surrealdb::engine::any::Any>,
}

fn get_from_env(key: &str) -> Result<String, Error> {
	std::env::var(key).map_err(|_| Error::MissingEnv { key: key.to_string() })
}

impl PersistenceEngineBuilder {
	pub fn new() -> Result<PersistenceEngineBuilder> {
		Ok(PersistenceEngineBuilder {
			ns: get_from_env("SURREAL_NAMESPACE")?,
			db: get_from_env("SURREAL_DATABASE")?,
			url: Url::parse(&get_from_env("SURREAL_ENDPOINT")?).unwrap(),
			user: get_from_env("SURREAL_USER")?,
			pass: get_from_env("SURREAL_PASSWORD")?,
		})
	}

	pub async fn connect(&self) -> Result<PersistenceEngine, Error> {
		let conn = surrealdb::engine::any::connect(self.url.as_str())
			.await
			.map_err(Error::InitialConnect)?;

		conn
			.use_ns(&self.ns)
			.use_db(&self.db)
			.await
			.map_err(Error::UseNsDb)?;

		conn.signin(surrealdb::opt::auth::Database {
			namespace: &self.ns,
			database: &self.db,
			username: &self.user,
			password: &self.pass,
		})
		.await
		.map_err(Error::Signin)?;

		// TODO: Add .e check meta info of connection
		// TODO: Implement caching for improved speed
		Ok(PersistenceEngine { conn })
	}
}

impl PersistenceEngine {
	pub fn
}
