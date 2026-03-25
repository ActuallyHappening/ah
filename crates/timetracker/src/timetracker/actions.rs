// use crate::prelude::*;

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
			let res: Vec<BillableCompany> = self.conn.insert("billable_company").content(company).await?;
			let res = res
				.into_iter()
				.next()
				.ok_or(eyre!("Should have only returned one record"))?;
			self.billable_companies.insert(res.id.clone(), res.clone());
			Ok(res)
		}
	}
}

pub use add_project::*;
mod add_project {
	use surrealdb::types::{RecordId, SurrealValue};

	use crate::{db::Db, prelude::*, timetracker::Project};

	#[derive(clap::Args)]
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

	#[derive(SurrealValue)]
	pub struct AddProject {
		pub billing_company: RecordId,
		pub proper_name: String,
		pub short_name: String,
	}

	impl Db {
		pub async fn add_project(&mut self, project: AddProject) -> Result<Project> {
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

pub use add_span::*;
mod add_span {
	use surrealdb::types::{RecordId, SurrealValue};

	use crate::{
		db::Db,
		prelude::*,
		timetracker::{Span, SpanType},
	};

	#[derive(SurrealValue)]
	pub struct AddSpan {
		pub r#type: SpanType,
		pub project: RecordId,
	}

	impl Db {
		pub async fn add_span(&mut self, span: AddSpan) -> Result<Span> {
			let res: Vec<Span> = self.conn.insert("span").content(span).await?;
			let res = res
				.into_iter()
				.next()
				.ok_or(eyre!("Should have only returned one record"))?;
			self.spans.insert(res.id.clone(), res.clone());
			Ok(res)
		}
	}
}
