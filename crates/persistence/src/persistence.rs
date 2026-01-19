use std::collections::{HashMap, HashSet};

use ah_tcita::{Ka_tcita, veciksi};
use serde::{Serialize, de::DeserializeOwned};
use surrealdb::opt::IntoResource;

use crate::{
	prelude::*,
	sidbo::{Sidbo, SidboTcita},
};

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
	std::env::var(key).map_err(|_| Error::MissingEnv {
		key: key.to_string(),
	})
}

impl PersistenceEngineBuilder {
	pub fn new() -> Result<PersistenceEngineBuilder> {
		Ok(PersistenceEngineBuilder {
			ns: get_from_env("SURREAL_NAMESPACE")?,
			db: get_from_env("SURREAL_DATABASE")?,
			url: Url::parse(&get_from_env("SURREAL_ENDPOINT")?).unwrap(),
			user: get_from_env("SURREAL_USER")?,
			pass: get_from_env("SURREAL_PASS")?,
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

		conn
			.signin(surrealdb::opt::auth::Database {
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
	pub async fn select_pasidbo_untyped(&self, id: SidboTcita) -> Result<Sidbo> {
		Ok(
			self
				.conn
				.select(&id.0)
				.await
				.map_err(|err| Error::SelectFailed {
					id: id.clone(),
					err,
				})?
				.ok_or(Error::SelectMissing { id })?,
		)
	}

	pub async fn select_pasidbo<Sidbo>(&self, id: SidboTcita) -> Result<Sidbo>
	where
		Sidbo: DeserializeOwned,
	{
		Ok(
			self
				.conn
				.select(&id.0)
				.await
				.map_err(|err| Error::SelectFailed {
					id: id.clone(),
					err,
				})?
				.ok_or(Error::SelectMissing { id })?,
		)
	}

	/// No order guarenteed
	pub async fn select_sidbo<TSidbo>(
		&self,
		ids: impl IntoIterator<Item = SidboTcita>,
	) -> Result<HashSet<crate::sidbo::Sidbo>>
	where
		TSidbo: TryFrom<crate::sidbo::Sidbo>,
		<TSidbo as TryFrom<crate::sidbo::Sidbo>>::Error: std::fmt::Debug,
	{
		let mut map = HashSet::new();
		for id in ids {
			map.insert(self.select_pasidbo_untyped(id).await?);
		}
		let map = map.into_iter().map(|sidbo| {
			let id = sidbo.get_id().clone();
			crate::sidbo::Sidbo::try_from(sidbo).map_err(|err| Error::SidboConversionFailed {
				id,
				err_debug: format!("{:?}", err),
			})
		});
		let map = map.collect::<Result<HashSet<_>>>()?;
		Ok(map)
	}

	pub async fn select_ckaji_sidbo<Ckaji>(&self) -> Result<HashSet<SidboTcita>>
	where
		Ckaji: Ka_tcita,
	{
		let mut resp = self
			.conn
			.query("SELECT id FROM sidbo WHERE ckaji.keys().matches($ckaji_tcita).any()")
			.bind(("ckaji_tcita", Ckaji::TCITA))
			.await
			.map_err(|err| Error::SelectCkaji {
				ckaji_tcita: Ckaji::TCITA.to_owned(),
				err,
			})?;
		let resp: Vec<SidboTcita> = resp.take(0).map_err(|err| Error::SelectCkaji {
			ckaji_tcita: Ckaji::TCITA.to_owned(),
			err,
		})?;
		Ok(resp.into_iter().collect())
	}
}

#[veciksi(lojban = "TODO builder", glico = "A builder to create a new Sidbo")]
pub struct SidboBuilder {
	pub name: String,
	ckaji: HashMap<String, serde_json::Value>,
}

impl SidboBuilder {
	pub fn new(name: String) -> Self {
		SidboBuilder {
			name,
			ckaji: Default::default(),
		}
	}

	pub fn add_ckaji<Ckaji>(mut self, ckaji: Ckaji) -> Result<Self>
	where
		Ckaji: Serialize + Ka_tcita,
	{
		self.ckaji.insert(
			Ckaji::TCITA.to_owned(),
			serde_json::to_value(ckaji).map_err(|err| Error::SerializingCkaji {
				ckaji_id: Ckaji::TCITA.to_owned(),
				err,
			})?,
		);
		Ok(self)
	}
}

#[derive(Serialize)]
struct RawSidbo {
	id: SidboTcita,
	ckaji: HashMap<String, serde_json::Value>,
}

impl PersistenceEngine {
	pub async fn add(&self, obj: SidboBuilder) -> Result<Sidbo> {
		let id = SidboTcita::from_name(&obj.name);
		let full_insert = RawSidbo {
			id: id.clone(),
			ckaji: obj.ckaji,
		};
		let ret: Option<Sidbo> = self
			.conn
			.insert((SidboTcita::TB, id.raw().key().clone()))
			.content(full_insert)
			.await
			.map_err(|err| Error::Add {
				id: id.clone(),
				err,
			})?;
		Ok(ret.expect("A just inserted record to exist"))
	}
}
