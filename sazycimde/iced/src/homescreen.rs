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
