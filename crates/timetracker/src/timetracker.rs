use std::collections::HashSet;

use ah_persistence::{
	PersistenceEngine, PersistenceEngineBuilder, SidboBuilder, sidbo::SidboTcita,
};
use color_eyre::eyre;

use crate::{
	prelude::*,
	timetracker_ckaji::{BillingCompanyCkaji, ProjectCkaji},
	timetracker_sidbo::{BillingCompanySidbo, ProjectSidbo, TimetrackerSidbo},
};

pub struct Timetracker {
	persistence: PersistenceEngine,
}

impl Timetracker {
	pub async fn new() -> Result<Self> {
		let persistence = PersistenceEngineBuilder::new()
			.wrap_err("Failed to create persistence engine")?
			.connect()
			.await?;
		Ok(Self { persistence })
	}

	pub async fn primary_sidbo(&self) -> Result<TimetrackerSidbo> {
		self
			.persistence
			.select_pasidbo(SidboTcita::from_tcita::<TimetrackerSidbo>())
			.await
			.wrap_err("Couldn't get primary sidbo")
	}

	pub async fn select_billing_company(&self, id: SidboTcita) -> Result<BillingCompanyCkaji> {
		self
			.persistence
			.select_pasidbo(SidboTcita::from_tcita::<BillingCompanyCkaji>())
			.await
			.wrap_err("Couldn't get billing company")
	}

	pub async fn get_billing_companies(&self) -> Result<Vec<BillingCompanySidbo>> {
		let ids = self
			.persistence
			.select_ckaji_sidbo::<BillingCompanyCkaji>()
			.await?;
		let companies = self.persistence.select_sidbo(ids).await?;
		Ok(companies)
	}
}

#[derive(clap::Args)]
pub struct AddBillingCompany {
	/// You must spell this exactly, then subsequent references to it are from short
	#[arg(long)]
	pub proper_name: String,
	#[arg(long)]
	pub short_name: String,
}

impl AddBillingCompany {
	/// time stodi
	pub fn tcita(&self) -> String {
		format!(
			"TODO ah-timetracker billing company {} noi aka {}",
			ah_lojban::quote(self.proper_name.as_str()),
			ah_lojban::quote(self.short_name.as_str())
		)
	}
}

impl Timetracker {
	pub async fn add_billing_company(
		&self,
		company: AddBillingCompany,
	) -> Result<BillingCompanySidbo> {
		// TODO: check that no billing companies with identical proper_name or short_name's exist

		let sidbo = SidboBuilder::new(company.tcita()).add_ckaji(BillingCompanyCkaji {
			proper_name: company.proper_name,
			short_name: company.short_name,
		})?;
		let mut sidbo = self.persistence.add(sidbo).await?;
		BillingCompanySidbo::try_from(sidbo)
	}
}

#[derive(clap::Args)]
pub struct AddProject {
	/// Long name here
	#[arg(long)]
	pub billing_company: String,

	/// You must spell this exactly, then subsequent references to it are from short
	#[arg(long)]
	pub proper_name: String,
	#[arg(long)]
	pub short_name: String,
}

impl AddProject {
	/// time stodi
	pub fn tcita(&self) -> String {
		format!(
			"TODO ah-timetracker project {} noi aka {}",
			ah_lojban::quote(self.proper_name.as_str()),
			ah_lojban::quote(self.short_name.as_str())
		)
	}
}

impl Timetracker {
	pub async fn add_project(&self, company: AddProject) -> Result<ProjectSidbo> {
		let companies = self.get_billing_companies().await?;
		let bcompany = companies
			.into_iter()
			.find(|c| c.ckaji().proper_name == company.proper_name);
		let company = bcompany.ok_or(eyre!(
			"No company with proper name {} found",
			company.proper_name
		))?;

		// TODO: check for duplicates

		let sidbo = SidboBuilder::new(company.tcita()).add_ckaji(ProjectCkaji {
			billing_company: company.id,
			proper_name: company.proper_name,
			short_name: company.short_name,
		})?;
		let mut sidbo = self.persistence.add(sidbo).await?;
		ProjectSidbo::try_from(sidbo)
	}
}
