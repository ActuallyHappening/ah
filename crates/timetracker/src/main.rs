use ah_timetracker::{cli::Cli, timetracker::Timetracker};
use clap::Parser as _;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
	// let cli = Cli::parse();

	let timetracker = Timetracker::new().await?;

	todo!()
}
