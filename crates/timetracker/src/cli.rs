use clap::{Parser, Subcommand};

use crate::timetracker::{AddBillingCompany, CliAddProject};

#[derive(Parser)]
#[command(name = "tt")]
#[command(bin_name = "tt")]
pub struct Cli {
	#[clap(subcommand)]
	pub cmd: SubCommands,
}

#[derive(Subcommand)]
pub enum SubCommands {
	Status,
	/// Used in my [starship](https://starship.rs/) config
	ShortStatus,
	/// Adds a new billing company
	Add(AddBillingCompany),
	/// Adds a new project
	New(CliAddProject),
	Start {
		#[arg(short = 'b')]
		billing_company_short: String,

		#[arg(short = 'p')]
		project_short: String,
	},
	Stop,
	/// Retrieves the active sub_description.
	/// This is useful for scripting
	Topic,
}
