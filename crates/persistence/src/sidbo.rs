use serde::{Deserialize, Serialize};

use crate::prelude::*;

/// Object
#[derive(Debug, Deserialize, Serialize)]
pub struct Sidbo {
	id: SidboTcita,
	ckaji: Vec<serde_json::Value>,
}

/// Don't optimize for space. Optimize for simplicity.
/// We could use just RecordIdKey here, but then deserialization is harder for no significant reason
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct SidboTcita(pub(crate) surrealdb::RecordId);
