use std::process;

use ah_timetracker::{
	cli::{Cli, SubCommands},
	timetracker::{
		CliAddProject, Timetracker,
		span::{StartBuilder, StopBuilder},
	},
};
use clap::Parser as _;
use color_eyre::eyre::eyre;
use time::UtcDateTime;
#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn};

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
	ah_timetracker::app_tracing::init_debug_tools("info,ah_timetracker=debug")?;
	let cli = Cli::parse();

	let timetracker = Timetracker::new().await?;

	match cli.cmd {
		SubCommands::Add(company) => {
			let company = timetracker.add_billing_company(company).await?;
			info!("Added billing company: {:?}", company);
			Ok(())
		}
		SubCommands::New(project) => {
			let project = CliAddProject::resolve(project, &timetracker).await?;
			let project = timetracker.add_project(project).await?;
			info!("Added new project: {:?}", project);
			Ok(())
		}
		SubCommands::Start {
			billing_company_short,
			project_short,
		} => {
			let companies = timetracker.get_billing_companies().await?;
			let company = companies
				.into_iter()
				.find(|c| c.ckaji().match_short_name(&billing_company_short))
				.ok_or(eyre!(
					"Couldn't find a company with short name `{}`",
					billing_company_short
				))?;
			let projects = timetracker.get_projects().await?;
			info!("Projects: {:?}", projects);
			let project = projects
				.into_iter()
				.find(|p| p.ckaji().match_short_name(&project_short))
				.ok_or(eyre!(
					"Couldn't find a project with short name `{}`",
					project_short
				))?;
			let start = StartBuilder {
				start: UtcDateTime::now(),
				project: project.tcita(),
				billing_company: company.tcita(),
			};
			timetracker.start(start).await?;

			Ok(())
		}
		SubCommands::Stop => {
			let spans = timetracker.get_spans().await?;
			let resolved = spans.resolve()?;
			for (company, project, state) in resolved.iter() {
				if let Some(_open) = state.open() {
					// stop this
					timetracker
						.stop(StopBuilder {
							stop: UtcDateTime::now(),
							billing_company: company.clone(),
							project: project.clone(),
						})
						.await?;
				}
			}

			Ok(())
		}
		SubCommands::Topic => {
			let spans = timetracker.get_spans().await?;
			let resolved = spans.resolve()?;
			let mut open = vec![];
			for (company, project, state) in resolved.iter() {
				if let Some(_open) = state.open() {
					open.push((company, project, state));
				}
			}
			let mut output = vec![];

			for (_c, p, _s) in open {
				let project = timetracker.get_project(p.clone()).await?;
				output.push(format!("{}", project.ckaji().proper_name()));
			}

			if output.is_empty() {
				error!("No open topics");
				process::exit(1);
			} else {
				let output = output.join(", ");
				println!("{}", output);
				Ok(())
			}
		}
	}
}
