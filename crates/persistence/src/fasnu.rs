use serde::{Deserialize, Serialize};
use surrealdb::Datetime;
use time::UtcDateTime;

use crate::{
	PersistenceEngine, SidboBuilder,
	prelude::*,
	sidbo::{Sidbo, SidboTcita},
};