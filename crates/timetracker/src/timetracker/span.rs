use ah_persistence::{PersistenceEngine, SidboBuilder, packaji_sidbo, sidbo::SidboTcita};

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

#[derive(Debug, Deserialize, Serialize)]
pub struct Start {
	#[serde(with = "time::serde::iso8601")]
	start: time::OffsetDateTime,
	project: SidboTcita,
	billing_company: SidboTcita,
}

#[derive(Debug, Deserialize, Serialize)]
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
			span::{FasnuCkaji, SpanFasnuSidbo, Start, Stop, StopBuilder},
		},
	};

	impl Timetracker {
		async fn get_spans_raw(&self) -> Result<Vec<SpanFasnuSidbo>> {
			let ids = self.persistence.select_ckaji_sidbo::<FasnuCkaji>().await?;
			let spans = self.persistence.select_sidbo(ids).await?;
			Ok(spans)
		}

		pub async fn get_spans(&self) -> Result<UnderstandSpanState> {
			let full = self.get_spans_raw().await?;
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

	pub struct ClosedSpan {
		pub(crate) start: Start,
		pub(crate) start_id: SidboTcita,
		pub(crate) stop: Stop,
		pub(crate) stop_id: SidboTcita,
	}

	pub struct OpenSpan {
		pub(crate) start: Start,
		pub(crate) start_id: SidboTcita,
	}

	impl UnderstandSpanState {
		pub fn resolve(self) -> Result<ResolvedSpanState> {
			let mut ret = ResolvedSpanState::default();
			for span in self.full {
				match span.ckaji() {
					FasnuCkaji::Start(start) => {
						ret
							.get(&start.billing_company, &start.project)
							.start(start)?;
					}
					FasnuCkaji::Stop(stop) => {}
				}
			}
			Ok(ret)
		}
	}

	impl SpansState {
		fn start(&mut self, start: &Start) -> Result<()> {
			todo!()
		}

		fn stop(&mut self, stop: &Stop) -> Result<()> {
			todo!()
		}
	}

	impl ResolvedSpanState {
		fn get(&mut self, company: &SidboTcita, project: &SidboTcita) -> &mut SpansState {
			self
				.inner
				.entry(company.clone())
				.or_default()
				.entry(project.clone())
				.or_default()
		}
	}
}
