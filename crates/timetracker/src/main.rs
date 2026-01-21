use ah_timetracker::{
	cli::{Cli, SubCommands},
	timetracker::{CliAddProject, Timetracker},
};
use clap::Parser as _;
use tracing::{debug, info};

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
	ah_timetracker::app_tracing::init_debug_tools("info,ah_timetracker=debug")?;
	let cli = Cli::parse();

	let timetracker = Timetracker::new().await?;
	let primary = timetracker.primary_sidbo().await?;
	debug!("Primary state: {:?}", primary);

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
		_ => todo!(),
	}
}
