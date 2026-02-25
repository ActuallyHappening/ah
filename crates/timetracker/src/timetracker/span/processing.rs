use std::collections::HashMap;

use ah_persistence::sidbo::SidboTcita;
use time::UtcOffset;

use crate::{
	prelude::*,
	timetracker::{
		Timetracker,
		span::{FasnuCkaji, SpanFasnuSidbo, Start, Stop},
	},
};

#[derive(Debug, Clone)]
pub struct WithSidboTcita<T> {
	inner: T,
	tcita: SidboTcita,
}

impl<T> WithSidboTcita<T> {
	pub fn inner(&self) -> &T {
		&self.inner
	}

	pub fn tcita(&self) -> SidboTcita {
		self.tcita.clone()
	}

	pub fn as_ref(&self) -> WithSidboTcita<&T> {
		WithSidboTcita {
			inner: &self.inner,
			tcita: self.tcita.clone(),
		}
	}
}

#[allow(dead_code)]
impl<T> WithSidboTcita<&T>
where
	T: Clone,
{
	fn clone_inner(self) -> WithSidboTcita<T> {
		WithSidboTcita {
			inner: self.inner.clone(),
			tcita: self.tcita.clone(),
		}
	}
}

impl Timetracker {
	async fn get_spans_raw(&self) -> Result<Vec<SpanFasnuSidbo>> {
		let ids = self.persistence.select_ckaji_sidbo::<FasnuCkaji>().await?;
		let spans = self.persistence.select_sidbo(ids).await?;
		Ok(spans)
	}

	/// Automatically sorts
	pub async fn get_spans(&self) -> Result<UnderstandSpanState> {
		let mut full = self.get_spans_raw().await?;
		full.sort_by(|a, b| a.ckaji().time().cmp(&b.ckaji().time()));
		Ok(UnderstandSpanState { full })
	}
}

pub struct UnderstandSpanState {
	full: Vec<SpanFasnuSidbo>,
}

#[derive(Debug, Default, Clone)]
pub struct ProjectResolved<T> {
	inner: HashMap<SidboTcita, HashMap<SidboTcita, T>>,
}
pub type ProjectResolvedSpanState = ProjectResolved<SpansState>;

#[derive(Default, Clone)]
pub struct SpansState {
	pub history: Vec<ClosedSpan>,
	pub open: Option<OpenSpan>,
}

pub type ClosedSpan = (WithSidboTcita<Start>, WithSidboTcita<Stop>);
pub type OpenSpan = WithSidboTcita<Start>;

impl UnderstandSpanState {
	pub fn resolve(self) -> Result<ProjectResolvedSpanState> {
		let mut ret = ProjectResolvedSpanState::default();
		for span in self.full {
			let tcita = span.tcita();
			match span.into_ckaji() {
				FasnuCkaji::Start(start) => {
					ret
						.get_mut(&start.billing_company, &start.project)
						.start(WithSidboTcita {
							inner: start,
							tcita: tcita,
						})?;
				}
				FasnuCkaji::Stop(stop) => {
					ret
						.get_mut(&stop.billing_company, &stop.project)
						.stop(WithSidboTcita {
							inner: stop,
							tcita: tcita,
						})?;
				}
			}
		}
		Ok(ret)
	}
}

impl SpansState {
	fn start(&mut self, start: WithSidboTcita<Start>) -> Result<()> {
		if self.open.is_some() {
			bail!("Cannot start a project that is already being worked on: {:?} {:?}", self.open, start);
		} else {
			self.open = Some(start);
			Ok(())
		}
	}

	fn stop(&mut self, stop: WithSidboTcita<Stop>) -> Result<()> {
		if let Some(open) = self.open.take() {
			self.history.push((open, stop));
			self.open = None;
			Ok(())
		} else {
			bail!("Cannot stop a project that is not being worked on: {:#?} {:?}", self.history, stop);
		}
	}
}

impl<T> ProjectResolved<T>
where
	T: Default,
{
	pub fn get_mut(&mut self, company: &SidboTcita, project: &SidboTcita) -> &mut T {
		self
			.inner
			.entry(company.clone())
			.or_default()
			.entry(project.clone())
			.or_default()
	}

	pub fn get(&self, company: &SidboTcita, project: &SidboTcita) -> Option<&T> {
		self.inner.get(company)?.get(project)
	}

	pub fn iter(&self) -> impl Iterator<Item = (&SidboTcita, &SidboTcita, &T)> {
		self.inner.iter().flat_map(|(company, projects)| {
			projects
				.iter()
				.map(move |(project, state)| (company, project, state))
		})
	}

	pub fn into_iter(self) -> impl Iterator<Item = (SidboTcita, SidboTcita, T)> {
		// ui la .clone.
		self.inner.into_iter().flat_map(|(company, projects)| {
			projects
				.into_iter()
				.map(move |(project, state)| (company.clone(), project, state))
		})
	}
}

impl<T> FromIterator<(SidboTcita, SidboTcita, T)> for ProjectResolved<T>
where
	T: Default,
{
	fn from_iter<I: IntoIterator<Item = (SidboTcita, SidboTcita, T)>>(iter: I) -> Self {
		let mut resolved = ProjectResolved::<T>::default();
		for (company, project, state) in iter {
			*resolved.get_mut(&company, &project) = state;
		}
		resolved
	}
}

impl SpansState {
	pub fn open(&self) -> Option<WithSidboTcita<&Start>> {
		self.open.as_ref().map(|open| open.as_ref())
	}
}

#[derive(Default, Clone, Debug)]
pub struct SpansByDay {
	pub clean: HashMap<Date, Vec<ClosedSpan>>,
	pub unclean: Vec<ClosedSpan>,
	pub open: Option<OpenSpan>,
}

impl SpansState {
	pub fn split_by_day(self, offset: UtcOffset) -> SpansByDay {
		let mut clean: HashMap<Date, Vec<ClosedSpan>> = HashMap::with_capacity(self.history.len());
		let mut unclean = Vec::with_capacity(0);

		for span in self.history {
			let (start, stop) = &span;
			let start_date = start.inner().start().to_offset(offset).date();
			let end_date = stop.inner().stop().to_offset(offset).date();
			if start_date != end_date {
				unclean.push(span);
			} else {
				clean.entry(start_date).or_default().push(span);
			}
		}

		SpansByDay {
			clean,
			unclean,
			open: self.open,
		}
	}
}

impl ProjectResolved<SpansState> {
	pub fn split_by_day(self, offset: UtcOffset) -> ProjectResolved<SpansByDay> {
		self
			.into_iter()
			.map(|(b, p, state)| (b, p, SpansState::split_by_day(state, offset)))
			.collect()
	}
}
