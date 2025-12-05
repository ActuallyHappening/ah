use std::{borrow::Borrow, marker::PhantomData};

use crate::lerfu::{CharNotLerfu, Lerfu};

#[derive(Default, Clone)]
pub struct ParserSettings {
	_none: PhantomData<()>,
}

#[derive(Default)]
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

	pub fn parse_lerfu(&self, input: char) -> Result<Lerfu, CharNotLerfu> {
		Lerfu::new(input)
	}
}

pub struct ParserResult {
	// pub paragraphs: Vec<Paragraph>,
}
