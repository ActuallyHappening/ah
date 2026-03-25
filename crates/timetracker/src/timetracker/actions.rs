use crate::prelude::*;

pub use add_billing_company::*;
mod add_billing_company {
	use surrealdb::types::SurrealValue;

	use crate::{db::Db, prelude::*, timetracker::BillableCompany};

	#[derive(clap::Args, SurrealValue)]
	pub struct AddBillingCompany {
		/// You must spell this exactly, then subsequent references to it are from short
		#[arg(long)]
		pub proper_name: String,
		#[arg(long)]
		pub short_name: String,
	}

	impl Db {
		pub async fn add_billing_company(
			&mut self,
			company: AddBillingCompany,
		) -> Result<BillableCompany> {
			let res: Vec<BillableCompany> = self.conn.insert("billing_company").content(company).await?;
			let res = res
				.into_iter()
				.next()
				.ok_or(eyre!("Should have only returned one record"))?;
			self.billable_companies.insert(res.id.clone(), res.clone());
			Ok(res)
		}
	}
}

mod add_project {
	use surrealdb::types::SurrealValue;

	use crate::{db::Db, prelude::*, timetracker::Project};

	#[derive(clap::Args, SurrealValue)]
	pub struct CliAddProject {
		/// Long name here
		#[arg(long)]
		pub billing_company: String,

		/// You must spell this exactly, then subsequent references to it are from short
		#[arg(long)]
		pub proper_name: String,
		#[arg(long)]
		pub short_name: String,
	}

	impl Db {
		pub async fn add_project(&mut self, project: CliAddProject) -> Result<Project> {
			let res: Vec<Project> = self.conn.insert("project").content(project).await?;
			let res = res
				.into_iter()
				.next()
				.ok_or(eyre!("Should have only returned one record"))?;
			self.projects.insert(res.id.clone(), res.clone());
			Ok(res)
		}
	}
}
