use ah_persistence::{SidboBuilder, packaji_sidbo, sidbo::SidboTcita};

use crate::{prelude::*, timetracker::Timetracker};

packaji_sidbo!(pub struct SpanFasnuSidbo {
	ckaji: FasnuCkaji,
	...
});

impl Ka_tcita for SpanFasnuSidbo {
	const TI_SELTCITA_BAU_LA_LOJBAN: &str = "TODO ah-timetracker fasnu span sidbo";
}

impl SpanFasnuSidbo {
	fn derive_name() -> String {
		format!(
			"{} noi randomness {}",
			SpanFasnuSidbo::TCITA,
			ah_lojban::quote(uuid::Uuid::new_v4().to_string()),
		)
	}
}

#[tcita("TODO ah-timetracker streaming fasnu ckaji")]
#[derive(Deserialize, Serialize, Debug)]
pub enum FasnuCkaji {
	#[serde(rename = "TODO ah-timetracker start")]
	Start(Start),
	#[serde(rename = "TODO ah-timetracker stop")]
	Stop(Stop),
}

impl FasnuCkaji {
	pub fn time(&self) -> time::UtcDateTime {
		match self {
			FasnuCkaji::Start(start) => start.start.into(),
			FasnuCkaji::Stop(stop) => stop.stop.into(),
		}
	}
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Start {
	#[serde(with = "time::serde::iso8601")]
	start: time::OffsetDateTime,
	project: SidboTcita,
	billing_company: SidboTcita,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Stop {
	#[serde(with = "time::serde::iso8601")]
	stop: time::OffsetDateTime,
	project: SidboTcita,
	billing_company: SidboTcita,
}

#[derive(Debug)]
pub struct StartBuilder {
	pub start: time::UtcDateTime,
	pub project: SidboTcita,
	pub billing_company: SidboTcita,
}

#[derive(Debug)]
pub struct StopBuilder {
	pub stop: time::UtcDateTime,
	pub project: SidboTcita,
	pub billing_company: SidboTcita,
}

impl Timetracker {
	#[instrument(skip_all)]
	pub async fn start(&self, fasnu: StartBuilder) -> Result<SpanFasnuSidbo> {
		let sidbo =
			SidboBuilder::new(SpanFasnuSidbo::derive_name()).add_ckaji(FasnuCkaji::Start(Start {
				start: UtcDateTime::now().into(),
				project: fasnu.project,
				billing_company: fasnu.billing_company,
			}))?;
		let sidbo = self
			.persistence
			.add(sidbo)
			.await
			.wrap_err("Couldn't start")?;
		let ret = SpanFasnuSidbo::try_from(sidbo)?;
		info!("Started a span");
		Ok(ret)
	}

	#[instrument(skip_all)]
	pub async fn stop(&self, fasnu: StopBuilder) -> Result<SpanFasnuSidbo> {
		let sidbo =
			SidboBuilder::new(SpanFasnuSidbo::derive_name()).add_ckaji(FasnuCkaji::Stop(Stop {
				stop: fasnu.stop.into(),
				project: fasnu.project,
				billing_company: fasnu.billing_company,
			}))?;
		let sidbo = self
			.persistence
			.add(sidbo)
			.await
			.wrap_err("Couldn't stop")?;
		let ret = SpanFasnuSidbo::try_from(sidbo)?;
		info!("Stopped a span");
		Ok(ret)
	}
}

pub mod processing {
	use std::collections::HashMap;

	use ah_persistence::sidbo::SidboTcita;

	use crate::{
		prelude::*,
		timetracker::{
			Timetracker,
			span::{FasnuCkaji, SpanFasnuSidbo, Start, Stop},
		},
	};

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

	#[derive(Default)]
	pub struct ResolvedSpanState {
		inner: HashMap<SidboTcita, HashMap<SidboTcita, SpansState>>,
	}

	#[derive(Default)]
	pub struct SpansState {
		history: Vec<ClosedSpan>,
		open: Option<OpenSpan>,
	}

	type ClosedSpan = (WithSidboTcita<Start>, WithSidboTcita<Stop>);
	type OpenSpan = WithSidboTcita<Start>;

	impl UnderstandSpanState {
		pub fn resolve(self) -> Result<ResolvedSpanState> {
			let mut ret = ResolvedSpanState::default();
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
				bail!("Cannot start a project that is already being worked on");
			} else {
				self.open = Some(start);
			}
			Ok(())
		}

		fn stop(&mut self, stop: WithSidboTcita<Stop>) -> Result<()> {
			if let Some(open) = self.open.take() {
				self.history.push((open, stop));
				self.open = None;
				Ok(())
			} else {
				bail!("Cannot stop a project that is not being worked on");
			}
		}
	}

	impl ResolvedSpanState {
		fn get_mut(&mut self, company: &SidboTcita, project: &SidboTcita) -> &mut SpansState {
			self
				.inner
				.entry(company.clone())
				.or_default()
				.entry(project.clone())
				.or_default()
		}

		pub fn get(&self, company: &SidboTcita, project: &SidboTcita) -> Option<&SpansState> {
			self.inner.get(company)?.get(project)
		}

		pub fn iter(&self) -> impl Iterator<Item = (&SidboTcita, &SidboTcita, &SpansState)> {
			self.inner.iter().flat_map(|(company, projects)| {
				projects
					.iter()
					.map(move |(project, state)| (company, project, state))
			})
		}
	}

	impl SpansState {
		pub fn open(&self) -> Option<WithSidboTcita<&Start>> {
			self.open.as_ref().map(|open| open.as_ref())
		}
	}
}
