use std::collections::HashMap;

use surrealdb::{Surreal, engine::any, types::RecordId};

use crate::{
	prelude::*,
	timetracker::{BillableCompany, Project, Span},
};

fn get_from_env(key: &str) -> Result<String> {
	std::env::var(key).wrap_err("Couldn't find an expected ENV variable")
}

#[derive(Debug, Clone)]
pub struct Db {
	pub(crate) conn: Surreal<any::Any>,
	pub billable_companies: HashMap<RecordId, BillableCompany>,
	pub projects: HashMap<RecordId, Project>,
	pub spans: HashMap<RecordId, Span>,
}

impl Db {
	pub async fn new() -> Result<Db> {
		let conn = Self::connect().await?;
		let mut ret = Self::empty(conn);
		ret.refresh_all().await?;
		Ok(ret)
	}

	fn empty(conn: Surreal<any::Any>) -> Db {
		Self {
			conn,
			billable_companies: Default::default(),
			projects: Default::default(),
			spans: Default::default(),
		}
	}

	async fn connect() -> Result<Surreal<any::Any>> {
		let creds = surrealdb::opt::auth::Database {
			namespace: get_from_env("SURREAL_NAMESPACE")?,
			database: get_from_env("SURREAL_DATABASE")?,
			username: get_from_env("SURREAL_USER")?,
			password: get_from_env("SURREAL_PASS")?,
		};
		let url = get_from_env("SURREAL_ENDPOINT")?;
		let conn = surrealdb::engine::any::connect(url).await?;

		conn.signin(creds).await?;
		Ok(conn)
	}

	async fn refresh_all(&mut self) -> Result<()> {
		self.refresh_billable_companies().await?;
		Ok(())
	}
}

mod billable_companies {
	use std::collections::HashMap;

	use surrealdb::types::RecordId;

	use crate::{db::Db, prelude::*, timetracker::BillableCompany};

	impl Db {
		pub(in crate::db) async fn refresh_billable_companies(&mut self) -> Result<()> {
			let companies: Vec<BillableCompany> = self.conn.select("billable_company").await?;
			let companies: HashMap<RecordId, BillableCompany> =
				companies.into_iter().map(|c| (c.id.clone(), c)).collect();
			self.billable_companies = companies;
			Ok(())
		}
	}
}

#[tokio::test]
async fn db_connects() -> Result<()> {
	let db = Db::new().await?;

	assert!(!db.billable_companies.is_empty());

	Ok(())
}
