use iced::Task;

use crate::prelude::*;

pub use state::*;
pub mod state {
	use iced::Task;

	use crate::{prelude::*, toplevel::TopLevelMessage};

	#[derive(Debug)]
	pub struct TopLevelState {
		theme: Theme,
		current: CurrentlyDisplaying,
		screens: ScreensState,
	}

	impl Default for TopLevelState {
		fn default() -> Self {
			TopLevelState {
				theme: Theme::TokyoNight,
				current: Default::default(),
				screens: ScreensState::default(),
			}
		}
	}

	impl TopLevelState {
		pub fn screen(&self) -> &ScreensState {
			&self.screens
		}

		pub fn screen_mut(&mut self) -> &mut ScreensState {
			&mut self.screens
		}

		pub fn current(&self) -> &CurrentlyDisplaying {
			&self.current
		}

		pub fn switch_screen_to(&mut self, new_screen: CurrentlyDisplaying) -> Task<TopLevelMessage> {
			self.current = new_screen;
			match &self.current {
				CurrentlyDisplaying::Home => ().into(),
				CurrentlyDisplaying::Timetracker => timetracker::State::start().map(Into::into),
			}
		}

		pub fn theme(&self) -> Theme {
			self.theme.clone()
		}
	}

	#[derive(Default, Debug)]
	pub struct ScreensState {
		pub home: (),
		pub timetracker: timetracker::State,
	}

	#[derive(Default, Debug, Clone)]
	pub enum CurrentlyDisplaying {
		#[default]
		Home,
		Timetracker,
	}
}

impl TopLevelState {
	pub fn update(&mut self, message: TopLevelMessage) -> Task<TopLevelMessage> {
		match message {
			TopLevelMessage::SwitchScreen(change) => self.switch_screen_to(change).into(),
			TopLevelMessage::Timetracker(msg) => self.screen_mut().timetracker.update(msg).into(),
		}
	}

	pub fn view(&self) -> Element<'_, TopLevelMessage> {
		match self.current() {
			CurrentlyDisplaying::Home => homescreen::home().map(TopLevelMessage::SwitchScreen),
			CurrentlyDisplaying::Timetracker => self.screen().timetracker.view(),
		}
	}
}

pub use message::*;
pub mod message {
	use crate::{prelude::*, toplevel::CurrentlyDisplaying};

	#[derive(Debug, Clone, From)]
	pub enum TopLevelMessage {
		SwitchScreen(CurrentlyDisplaying),
		Timetracker(timetracker::Message),
	}
}
