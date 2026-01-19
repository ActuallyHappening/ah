use clap::{Parser, Subcommand};

use crate::timetracker::AddBillingCompany;

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
	/// Use like `tt start - -` for repeat
	Start {
		#[arg(short = 'b')]
		billing_company_short: Option<String>,

		#[arg(short = 'p', short = 't')]
		project_tcita: Option<String>,

		/// Will assume your sub-description is new
		#[clap(short, long)]
		new: bool,
	},
	Stop,
	/// Retrieves the active sub_description.
	/// This is useful for scripting
	Topic,
}
