use iced::Task;

	use crate::prelude::*;

	#[derive(Debug, Clone)]
	pub struct TopLevelState {
		theme: Theme,
		screen: ScreensState,
	}

	impl Default for TopLevelState {
		fn default() -> Self {
			TopLevelState {
				theme: Theme::TokyoNight,
				screen: ScreensState::Home,
			}
		}
	}

	#[derive(Debug, Clone)]
	pub(crate) enum ScreensState {
		Home,
		Timetracker(timetracker::State),
	}

	impl TopLevelState {
		pub fn update(&mut self, message: TopLevelMessage) -> Task<TopLevelMessage> {
			match message {
				TopLevelMessage::Home(change) => match change {
					homescreen::ChangeScreens::Timetracker => {
						self.screen = ScreensState::Timetracker(timetracker::State::default());
						().into()
					}
				},
				TopLevelMessage::Home(home) =>
			}
		}

		pub fn view(&self) -> Element<'_, TopLevelMessage> {
			match &self.screen {
				ScreensState::Home => homescreen::home().map(TopLevelMessage::Home),
				ScreensState::Timetracker(state) => state.view(),
			}
		}

		pub fn theme(&self) -> Theme {
			self.theme.clone()
		}
	}

	pub use message::*;
	pub mod message {
		use crate::prelude::*;

		#[derive(Debug, Clone, From)]
		pub enum TopLevelMessage {
			Home(homescreen::ChangeScreens),
			Timetracker(timetracker::Message),
		}
	}
