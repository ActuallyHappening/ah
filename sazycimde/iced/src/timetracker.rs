use std::collections::HashMap;

use ah_timetracker::{
	sidbo::SidboTcita,
	timetracker::{
		Timetracker,
		span::processing::{ProjectResolved, ProjectResolvedSpanState, SpansByDay, SpansState},
	},
	timetracker_sidbo::{BillingCompanySidbo, ProjectSidbo},
};
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
	company_selector: Option<widget::combo_box::State<IdSelector>>,
	company: Option<IdSelector>,
	project_selector: Option<widget::combo_box::State<IdSelector>>,
	project: Option<IdSelector>,
	data: Option<Result<Data, String>>,
}

#[derive(Debug, Clone)]
struct IdSelector {
	id: SidboTcita,
	proper_name: String,
}

impl std::fmt::Display for IdSelector {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.proper_name)
	}
}

#[derive(Debug, Clone)]
struct Data {
	by_project: ProjectResolved<SpansByDay>,
	billing_companies: Vec<BillingCompanySidbo>,
	projects: Vec<ProjectSidbo>,
}

impl State {
	fn data<'s>(
		&'s self,
		f: impl FnOnce(&'s Data) -> Element<'s, TopLevelMessage>,
	) -> Element<'s, TopLevelMessage> {
		match &self.data {
			None => text("Loading").into(),
			Some(Err(err)) => text(err.to_owned()).into(),
			Some(Ok(data)) => f(&data),
		}
	}

	fn load_data(&mut self, data: Result<Data, String>) {
		if let Ok(data) = &data {
			self.company_selector = Some(widget::combo_box::State::new(
				data
					.billing_companies
					.iter()
					.map(|c| IdSelector {
						proper_name: c.ckaji().proper_name().to_owned(),
						id: c.tcita(),
					})
					.collect(),
			));

			self.project_selector = Some(widget::combo_box::State::new(
				data
					.projects
					.iter()
					.map(|c| IdSelector {
						proper_name: c.ckaji().proper_name().to_owned(),
						id: c.tcita(),
					})
					.collect(),
			));
		}

		self.data = Some(data);
	}

	fn company_selector<'s>(
		&'s self,
		f: impl FnOnce(&'s widget::combo_box::State<IdSelector>) -> Element<'s, TopLevelMessage>,
	) -> Element<'s, TopLevelMessage> {
		self
			.company_selector
			.as_ref()
			.map(f)
			.unwrap_or(text("Loading ...").into())
	}

	fn project_selector<'s>(
		&'s self,
		f: impl FnOnce(&'s widget::combo_box::State<IdSelector>) -> Element<'s, TopLevelMessage>,
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
	Loaded(Result<Data, String>),
	SelectCompany(IdSelector),
	SelectProject(IdSelector),
}

impl State {
	pub(crate) fn start() -> Task<Message> {
		let offset = UtcOffset::from_hms(10, 0, 0).expect("+10 UTC");
		Task::perform(fetch_data(offset), |data| {
			Message::Loaded(data.map_err(|err| err.to_string()))
		})
	}

	pub(crate) fn view(&self) -> Element<'_, TopLevelMessage> {
		let company_picker = self.company_selector(|state| {
			widget::ComboBox::new(
				&state,
				"Select Billing Company",
				self.company.as_ref(),
				|m| TopLevelMessage::Timetracker(Message::SelectCompany(m)),
			)
			.into()
		});
		let project_picker = self.project_selector(|state| {
			widget::ComboBox::new(&state, "Select Project", self.company.as_ref(), |m| {
				TopLevelMessage::Timetracker(Message::SelectProject(m))
			})
			.into()
		});
		let header = row![
			button("<- Back").on_press(CurrentlyDisplaying::Home.into()),
			text("Timetracker"),
			company_picker,
			project_picker
		];
		let body = self.data(|data| {
			let Some(company) = &self.company else {
				return text("No company selected").into();
			};
			let Some(project) = &self.project else {
				return text("No project selected").into();
			};
			let Some(data) = data.by_project.get(&company.id, &project.id) else {
				return text("No data for company/project combo").into();
			};
			Self::body(data)
		});

		widget::column![header, body].into()
	}

	pub(crate) fn update(&mut self, message: Message) {
		match message {
			Message::Loaded(data) => self.load_data(data),
			Message::SelectCompany(company) => self.company = Some(company),
			Message::SelectProject(project) => self.project = Some(project),
		}
	}

	fn body(data: &SpansByDay) -> Element<'static, TopLevelMessage> {
		let offset = UtcOffset::from_hms(10, 0, 0).expect("+10 UTC");

		const DAYS: usize = 7;
		let mut per_day = widget::Row::with_capacity(DAYS);
		let today = UtcDateTime::now().to_offset(offset).date();
		let past_week = (0..DAYS as i64).map(|day| today - day.days());

		fn day(date: Date) -> String {
			// english centric impl
			date.weekday().to_string()
		}

		fn duration(duration: Duration) -> String {
			let hours = duration.whole_hours();
			let minutes = duration.whole_minutes() % 60;
			let seconds = duration.whole_seconds() % 60;
			format!("{}h {}m {}s", hours, minutes, seconds)
		}

		for date in past_week {
			let mut col = widget::Column::new();
			col = col.push(text!("{} {}", day(date), date.day()));
			if let Some(clean) = data.clean.get(&date) {
				for span in clean {
					let durationxipa = span.1.inner().stop() - span.0.inner().start();
					col = col.push(text!("{}", duration(durationxipa)));
				}
				per_day = per_day.push(col);
			}
		}

		per_day.into()
	}
}

async fn fetch_data(offset: UtcOffset) -> color_eyre::Result<Data> {
	info!("Fetching data");
	let timetracker = Timetracker::new().await?;
	let spans = timetracker.get_spans().await?.resolve()?;
	info!("Fetched spans");
	// uinai hard coded +10
	let by_project = spans.split_by_day(offset);
	let billing_companies = timetracker.get_billing_companies().await?;
	info!("fetched billing companies");
	let projects = timetracker.get_projects().await?;
	info!("fetched projects");
	Ok(Data {
		by_project,
		billing_companies,
		projects,
	})
}
