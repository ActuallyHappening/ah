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
				TopLevelMessage::ChangeScreen(change) => match change {
					homescreen::ChangeScreens::Timetracker => self.screen = ScreensState::Timetracker,
				},
			}
		}

		pub fn view(&self) -> Element<'_, TopLevelMessage> {
			match &self.screen {
				ScreensState::Home => homescreen::home().map(TopLevelMessage::ChangeScreen),
				ScreensState::Timetracker => timetracker::view(),
			}
		}

		pub fn theme(&self) -> Theme {
			self.theme.clone()
		}
	}

	#[derive(Debug, Clone)]
	pub enum TopLevelMessage {
		ChangeScreen(homescreen::ChangeScreens),
	}
}

pub mod homescreen {
	use iced::Length::Fill;

	use crate::prelude::*;
	use crate::toplevel::{ScreensState, TopLevelMessage};

	#[derive(Debug, Clone)]
	pub(crate) enum ChangeScreens {
		Timetracker,
	}

	pub(crate) fn home() -> Element<'static, ChangeScreens> {
		button("Timetracker")
			.width(Fill)
			.height(Fill)
			.on_press(ChangeScreens::Timetracker)
			.into()
	}
}

pub mod timetracker {
	use std::collections::HashMap;

	use ah_timetracker::timetracker::span::processing::{ProjectResolved, ProjectResolvedSpanState, SpansState};
	use iced::widget::Row;
	use time::Date;

	use crate::{prelude::*, toplevel::TopLevelMessage};

	pub(crate) struct State {
		by_project: ProjectResolved<SpansByDay>
	}

	pub struct SpansByDay {
		clean: HashMap<Date, SpansState>,
		unclean: SpansState,
	}
	
	impl State {
		fn split_by_day(project_resolved: &ProjectResolvedSpanState) -> ProjectResolved<SpansByDay> {
				let mut row = Row::with_capacity(times.len());
				todo!()
		}
	}

	pub(crate) fn view(state: &State) -> Element<'static, TopLevelMessage> {
		let times = vec![1, 2, 3];
		todo!()
	}
}
