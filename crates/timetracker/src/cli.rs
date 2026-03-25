use clap::{Parser, Subcommand};

use crate::timetracker::actions::{AddBillingCompany, CliAddProject};

#[derive(Parser)]
#[command(name = "tt")]
#[command(bin_name = "tt")]
pub struct Cli {
	#[clap(subcommand)]
	pub cmd: SubCommands,
}

#[derive(Subcommand)]
pub enum SubCommands {
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
	/// Retrieves the active project proper name.
	/// Used in starship config.
	Topic,
	Dump,
}
