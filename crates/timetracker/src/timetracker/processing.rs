use surrealdb::types::RecordId;
use time::UtcOffset;

use crate::{
	db::Db,
	prelude::*,
	timetracker::{Span, SpanType},
};

pub type ClosedSpan = Span;
pub type OpenSpan = Span;

impl Db {
	pub fn day(&self, project: &RecordId, day: Date, offset: UtcOffset) -> Vec<Duration> {
		let mut spans: Vec<&Span> = self
			.spans
			.values()
			.filter(|s| &s.project == project)
			.filter(|s| s.time().to_offset(offset).date() == day)
			.collect();
		spans.sort_by(|a, b| a.time().cmp(&b.time()));
		let mut past_durations = Vec::with_capacity(spans.len() / 2);
		let mut open_start = None;

		for span in &spans {
			match span.r#type {
				SpanType::Start => {
					if open_start.is_some() {
						warn!(?project, ?day, "Ignoring a previously open span")
					}
					open_start = Some(span.time().to_offset(offset));
				}
				SpanType::Stop => {
					if let Some(start) = open_start {
						let end = span.time().to_offset(offset);
						past_durations.push(end - start);
						open_start = None;
					} else {
						warn!(?project, ?day, "Ignoring a stop span with no open start")
					}
				}
			}
		}

		past_durations
	}

	pub fn open(&self) -> Vec<RecordId> {
		let mut ret = Vec::with_capacity(self.projects.len());

		let mut spans: Vec<&Span> = self.spans.values().collect();
		// older -> newest
		spans.sort_by(|a, b| a.time().cmp(&b.time()));
		for project in self.projects.keys() {
			let spans = spans.iter().filter(|s| &s.project == project);
			if let Some(span) = spans.last() {
				if span.r#type == SpanType::Start {
					// latest span was a start
					ret.push(project.clone());
				}
			}
		}

		ret
	}
}

// #[derive(Default, Clone, Debug)]
// pub struct SpansByDay {
// 	pub clean: HashMap<Date, Vec<ClosedSpan>>,
// 	pub unclean: Vec<ClosedSpan>,
// 	pub open: Option<OpenSpan>,
// }

// impl SpansState {
// 	pub fn split_by_day(self, offset: UtcOffset) -> SpansByDay {
// 		let mut clean: HashMap<Date, Vec<ClosedSpan>> = HashMap::with_capacity(self.history.len());
// 		let mut unclean = Vec::with_capacity(0);

// 		for span in self.history {
// 			let (start, stop) = &span;
// 			let start_date = start.inner().start().to_offset(offset).date();
// 			let end_date = stop.inner().stop().to_offset(offset).date();
// 			if start_date != end_date {
// 				unclean.push(span);
// 			} else {
// 				clean.entry(start_date).or_default().push(span);
// 			}
// 		}

// 		SpansByDay {
// 			clean,
// 			unclean,
// 			open: self.open,
// 		}
// 	}
// }
