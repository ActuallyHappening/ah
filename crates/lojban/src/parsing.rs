use std::marker::PhantomData;

#[derive(Default, Clone)]
pub struct ParserSettings {
	_none: PhantomData<()>,
}

pub struct Parser {
	pub settings: ParserSettings,
	_unknown: PhantomData<()>,
}

impl Parser {
	pub fn new(settings: ParserSettings) -> Self {
		Self {
			settings,
			_unknown: PhantomData,
		}
	}
}

impl Parser {
	pub fn parse(&self, input: impl Borrow<str>) -> ParserResult {
		todo!()
	}
}

pub struct ParserResult {
	pub paragraphs: Vec<Paragraph>,
}
