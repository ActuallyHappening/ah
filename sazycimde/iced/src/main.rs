use ah_sazycimde_iced::toplevel::TopLevelState;
use ah_timetracker::app_tracing;
use color_eyre::eyre::Context as _;
#[allow(unused)]
use tracing::debug;

fn main() -> color_eyre::Result<()> {
	app_tracing::init_debug_tools("warn,ah_sazycimde_iced=debug,ah_timetracker=info")?;
	debug!("Started tracing");

	iced::application(
		TopLevelState::default,
		TopLevelState::update,
		TopLevelState::view,
	)
	.theme(TopLevelState::theme)
	.run()
	.wrap_err("iced error")
}
