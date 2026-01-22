pub mod prelude {
	pub(crate) use iced::{Element, Theme, widget::button};

	pub(crate) use crate::*;
}
pub mod toplevel {
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
		Timetracker,
	}

	impl TopLevelState {
		pub fn update(&mut self, message: TopLevelMessage) {
			match message {
				TopLevelMessage::Home(change) => match change {
					homescreen::ChangeScreens::Timetracker => self.screen = ScreensState::Timetracker,
				},
			}
		}

		pub fn view(&self) -> Element<'_, TopLevelMessage> {
			match &self.screen {
				ScreensState::Home => homescreen::home().map(TopLevelMessage::Home),
				ScreensState::Timetracker => timetracker::timetracker(),
			}
		}

		pub fn theme(&self) -> Theme {
			self.theme.clone()
		}
	}

	#[derive(Debug, Clone)]
	pub enum TopLevelMessage {
		Home(homescreen::ChangeScreens)
	}
}

pub mod homescreen {
	pub use crate::prelude::*;
	use crate::toplevel::{ScreensState, TopLevelMessage};
	
	#[derive(Debug, Clone)]
	pub(crate) enum ChangeScreens {
		Timetracker,
	}

	pub(crate) fn home() -> Element<'static, ChangeScreens> {
		button("Timetracker").on_press(ChangeScreens::Timetracker).into()
	}
}

pub mod timetracker {
	use crate::{prelude::*, toplevel::TopLevelMessage};
	
	// pub(crate) 

	pub(crate) fn timetracker() -> Element<'static, TopLevelMessage> {
		button("this is your timetracker").into()
	}
}
