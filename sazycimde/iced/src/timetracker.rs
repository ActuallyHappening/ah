use std::collections::HashMap;

use ah_timetracker::{RecordId, db::Db, timetracker::Project};
use iced::{
	Task,
	widget::{Row, column, text},
};
use time::{Date, Duration, UtcDateTime, UtcOffset, ext::NumericalDuration, format_description};

use crate::{
	prelude::*,
	toplevel::{CurrentlyDisplaying, TopLevelMessage},
};

#[derive(Default, Debug)]
pub struct State {
	project_selector: Option<widget::combo_box::State<Project>>,
	project: Option<Project>,
	data: Option<Result<Db, String>>,
}

// #[derive(Debug, Clone)]
// struct Data {
// 	by_project: ProjectResolved<SpansByDay>,
// 	billing_companies: Vec<BillingCompanySidbo>,
// 	projects: Vec<ProjectSidbo>,
// }

impl State {
	fn data<'s>(
		&'s self,
		f: impl FnOnce(&'s Db) -> Element<'s, TopLevelMessage>,
	) -> Element<'s, TopLevelMessage> {
		match &self.data {
			None => text("Loading").into(),
			Some(Err(err)) => text(err.to_owned()).into(),
			Some(Ok(data)) => f(&data),
		}
	}

	fn load_data(&mut self, data: Result<Db, String>) {
		if let Ok(data) = &data {
			self.project_selector = Some(widget::combo_box::State::new(
				data.projects.values().cloned().collect(),
			));
		}

		self.data = Some(data);
	}

	fn project_selector<'s>(
		&'s self,
		f: impl FnOnce(&'s widget::combo_box::State<Project>) -> Element<'s, TopLevelMessage>,
	) -> Element<'s, TopLevelMessage> {
		self
			.project_selector
			.as_ref()
			.map(f)
			.unwrap_or(text("Loading ...").into())
	}
}

#[allow(private_interfaces)]
#[derive(Debug, Clone)]
pub enum Message {
	Loaded(Result<Db, String>),
	SelectProject(Project),
}

impl State {
	pub(crate) fn start() -> Task<Message> {
		Task::perform(fetch_data(), |data| {
			Message::Loaded(data.map_err(|err| err.to_string()))
		})
	}

	pub(crate) fn view(&self) -> Element<'_, TopLevelMessage> {
		let project_picker = self.project_selector(|state| {
			widget::ComboBox::new(&state, "Select Project", self.project.as_ref(), |m| {
				TopLevelMessage::Timetracker(Message::SelectProject(m))
			})
			.into()
		});
		let header = row![
			button("<- Back").on_press(CurrentlyDisplaying::Home.into()),
			text("Timetracker"),
			project_picker
		];
		let body = self.data(|data| {
			let Some(project) = &self.project else {
				return text("No project selected").into();
			};
			Self::body(data, project)
		});

		widget::column![header, body].into()
	}

	pub(crate) fn update(&mut self, message: Message) {
		match message {
			Message::Loaded(data) => self.load_data(data),
			Message::SelectProject(project) => self.project = Some(project),
		}
	}

	fn body(data: &Db, project: &Project) -> Element<'static, TopLevelMessage> {
		let offset = UtcOffset::from_hms(10, 0, 0).expect("+10 UTC");

		const DAYS: usize = 7;
		let mut per_day = widget::Row::with_capacity(DAYS);
		let today = UtcDateTime::now().to_offset(offset).date();
		let past_week = (0..DAYS as i64).rev().map(|day| today - day.days());

		fn day(date: Date) -> String {
			// english centric impl
			date.weekday().to_string()
		}

		fn fmt_duration(duration: Duration) -> String {
			let hours = duration.whole_hours();
			let minutes = duration.whole_minutes() % 60;
			let seconds = duration.whole_seconds() % 60;
			format!("{}h {}m {}s", hours, minutes, seconds)
		}

		let mut week_total = Duration::ZERO;
		for date in past_week {
			let mut col = widget::Column::new();
			col = col.push(text!("{} {}", day(date), date.day()));
			let mut day_total = Duration::ZERO;
			let clean = data.day(&project.id, date, offset);
			for duration in clean {
				day_total += duration;
				col = col.push(text!("{}", fmt_duration(duration)));
			}
			week_total += day_total;
			col = col.push(text!("Total: {}", fmt_duration(day_total)));
			per_day = per_day.push(col);
		}
		per_day = per_day.push(text!("Week total: {}", fmt_duration(week_total)));

		per_day.into()
	}
}

async fn fetch_data() -> color_eyre::Result<Db> {
	info!("Fetching data");
	let db = Db::new().await?;
	Ok(db)
}
