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
pub(crate) struct State {
	by_project: Option<ProjectResolved<SpansByDay>>,
}

#[derive(Debug, Clone)]
pub(crate) enum Message {
	Loaded(ProjectResolved<SpansByDay>),
}

impl State {
	pub(crate) fn start() -> Task<TopLevelMessage> {
		Task::done(Message::Loaded(Default::default()).into())
	}
	pub(crate) fn view(&self) -> Element<'static, TopLevelMessage> {
		text!("{:?}", self.by_project).into()
	}

	pub(crate) fn update(&mut self, message: Message) {}
}

pub async fn fetch_spans() -> color_eyre::Result<ProjectResolved<SpansByDay>> {
	let timetracker = Timetracker::new().await?;
	let spans = timetracker.get_spans().await?.resolve()?;
	// uinai hard coded +10
	Ok(spans.split_by_day(UtcOffset::from_hms(10, 0, 0)?))
}
