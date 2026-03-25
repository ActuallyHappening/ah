use std::process;

use ah_timetracker::{
	cli::{Cli, SubCommands}, db::Db, timetracker::{BillableCompany, SpanType, actions::{AddSpan, CliAddProject}}
};
use clap::Parser as _;
use color_eyre::eyre::eyre;
#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn};

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
	ah_timetracker::app_tracing::init_debug_tools("info,ah_timetracker=debug")?;
	let cli = Cli::parse();

	let mut timetracker = Db::new().await?;

	match cli.cmd {
		SubCommands::Add(company) => {
			let company = timetracker.add_billing_company(company).await?;
			info!("Added billing company: {:?}", company);
			Ok(())
		}
		SubCommands::New(CliAddProject { billing_company, proper_name, short_name }) => {
			let company: &BillableCompany= timetracker.billable_companies.values().find(|b| b.proper_name == billing_company).ok_or(eyre!("Couldn't find a company with the given proper name"))?;
			let project = timetracker.add_project(ah_timetracker::timetracker::actions::AddProject { billing_company: company.id.clone(), proper_name, short_name }).await?;
			info!("Added new project: {:?}", project);
			Ok(())
		}
		SubCommands::Start {
			billing_company_short,
			project_short,
		} => {
			let companies = timetracker.billable_companies.values();
			let company = companies
				.into_iter()
				.find(|c| c.short_name == billing_company_short)
				.ok_or(eyre!(
					"Couldn't find a company with short name `{}`",
					billing_company_short
				))?;

			let projects = timetracker.projects.values();
			let project = projects
				.into_iter()
				.find(|p| p.short_name == project_short)
				.ok_or(eyre!(
					"Couldn't find a project with short name `{}`",
					project_short
				))?;

			info!("Starting span for company {} and project {}", company, project);
			timetracker.add_span(AddSpan {
				r#type: SpanType::Start,
				project: project.id.clone(),
			}).await?;

			Ok(())
		}
		SubCommands::Stop => {
			for open in timetracker.open() {
				let project = timetracker.projects.get(&open).unwrap();
				info!(%project, "Closing span for project");
				timetracker.add_span(AddSpan { r#type: SpanType::Stop, project: open.clone() }).await?;
			}

			Ok(())
		}
		SubCommands::Topic => {
			let open = timetracker.open();
			if open.is_empty() {
				info!("No open spans");
				process::exit(1)
			} else {
				let mut output = Vec::with_capacity(open.len());
				for open in open {
					let project = timetracker.projects.get(&open).unwrap();
					output.push(format!("{}", project));
				}
				println!("{}", output.join(", "));
				Ok(())
			}
		}
		SubCommands::Dump => {
			println!("{:#?}", timetracker.projects);

			Ok(())
		}
	}
}
