use std::collections::HashMap;

use ah_timetracker::timetracker::{
	Timetracker,
	span::processing::{ProjectResolved, ProjectResolvedSpanState, SpansByDay, SpansState},
};
use iced::{
	Task,
	widget::{Row, text},
};
use time::{Date, UtcOffset};

use crate::{prelude::*, toplevel::TopLevelMessage};

#[derive(Default, Clone, Debug)]
pub struct State {
	by_project: Option<ProjectResolved<SpansByDay>>,
	err: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Message {
	Loaded(ProjectResolved<SpansByDay>),
	Err(String),
}

impl State {
	pub(crate) fn start() -> Task<Message> {
		Task::perform(fetch_spans(), |data| match data {
			Ok(data) => Message::Loaded(data),
			Err(err) => Message::Err(err.to_string()),
		})
	}

	pub(crate) fn view(&self) -> Element<'static, TopLevelMessage> {
		widget::column![text!("{:?}", self.by_project), text!("{:?}", self.err)].into()
	}

	pub(crate) fn update(&mut self, message: Message) {
		match message {
			Message::Loaded(data) => self.by_project = Some(data),
			Message::Err(err) => self.err = Some(err),
		}
	}
}

pub async fn fetch_spans() -> color_eyre::Result<ProjectResolved<SpansByDay>> {
	info!("Fetching data");
	let timetracker = Timetracker::new().await?;
	let spans = timetracker.get_spans().await?.resolve()?;
	// uinai hard coded +10
	Ok(spans.split_by_day(UtcOffset::from_hms(10, 0, 0)?))
}
