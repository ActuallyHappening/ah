use ah_timetracker::{cli::Cli, timetracker::Timetracker};
use clap::Parser as _;
use tracing::debug;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
	ah_timetracker::app_tracing::init_debug_tools("info,ah_timetracker=debug")?;
	// let cli = Cli::parse();

	let timetracker = Timetracker::new().await?;
	let primary = timetracker.primary_sidbo().await?;
	debug!("Primary state: {:?}", primary);

	todo!()
}
