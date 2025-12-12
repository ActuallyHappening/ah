use std::collections::HashMap;

use color_eyre::eyre::{Context as _, bail};
use compact_str::CompactString;
use serde::Deserialize;

use crate::valsi::{Cmavo, Valsi};

// const JBOVLASTE_GLICO_JSON: serde_json::Value = include_json::include_json!(concat!(
// 	env!("CARGO_MANIFEST_DIR"),
// 	"/../../downloads/dictionary-en.json"
// ));
const JBOVLASTE_GLICO_STR: &str = include_str!(concat!(
	env!("CARGO_MANIFEST_DIR"),
	"/../../downloads/dictionary-en.json"
));

#[derive(Deserialize)]
struct RawJbovlasteEntry {
	word: RawWord,
	word_type: WordType,
	selmaho: Option<CompactString>,
	definition: String,
	definition_id: u32,
	notes: CompactString,
	// score: f64,
}

#[derive(Deserialize)]
struct RawWord(CompactString);

/// open downloads/dictionary-en.json | get word_type | uniq
#[derive(Deserialize)]
#[serde(try_from = "String")]
enum WordType {
	Gismu,
	Cmavo,
	Lujvo,
	Cmevla,
	BuLetteral,
	ZeiLujvo,
	Fuhivla,
	CmavoCompound,
	ExperimentalGismu,
	ExperimentalCmavo,
	ObsoleteCmavo,
	ObsoleteCmevla,
	ObsoleteFuhivla,
	ObsoleteZeiLujvo,
}

impl TryFrom<String> for WordType {
	type Error = color_eyre::Report;
	fn try_from(value: String) -> Result<Self, Self::Error> {
		Ok(match value.as_str() {
			"gismu" => Self::Gismu,
			"cmavo" => Self::Cmavo,
			"lujvo" => Self::Lujvo,
			"cmevla" => Self::Cmevla,
			"bu-letteral" => Self::BuLetteral,
			"zei-lujvo" => Self::ZeiLujvo,
			"fu'ivla" => Self::Fuhivla,
			"cmavo-compound" => Self::CmavoCompound,
			"experimental cmavo" => Self::ExperimentalCmavo,
			"obselete cmavo" => Self::ObsoleteCmavo,
			"obselete cmevla" => Self::ObsoleteCmevla,
			"obselete zei-lujvo" => Self::ObsoleteZeiLujvo,
			_ => bail!("Unknown word_type {}", &value),
		})
	}
}

struct ValsiEntry {
	solmaho: Option<CompactString>,
	definition: String,
	notes: CompactString,
}

pub struct Jbolvlaste_glico {
	by_word_type: HashMap<WordType, HashMap<CompactString, ValsiEntry>>,
}

impl Jbolvlaste_glico {
	pub fn from_embedded() -> color_eyre::Result<Self> {
		let json: Vec<RawJbovlasteEntry> = serde_json::from_str(JBOVLASTE_GLICO_STR)
			.wrap_err("Couldn't parse str into raw jbovvlaste entry")?;
		// open downloads/dictionary-en.json | get word_type | uniq | count
		let mut by_word_type = HashMap::with_capacity(14);
	}
}
