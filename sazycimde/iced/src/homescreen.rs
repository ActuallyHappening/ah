use iced::Length::Fill;

use crate::{prelude::*, toplevel::CurrentlyDisplaying};

pub(crate) fn home() -> Element<'static, CurrentlyDisplaying> {
	button("Timetracker")
		.width(Fill)
		.height(Fill)
		.on_press(CurrentlyDisplaying::Timetracker)
		.into()
}
