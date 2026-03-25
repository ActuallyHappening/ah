use std::collections::{HashMap, HashSet};

use surrealdb::types::RecordId;

use crate::{prelude::*, timetracker::BillableCompany};

pub struct Db {
	conn: surrealdb::SurrealDb<any::Any>,
	billable_companies: HashMap<RecordId, BillableCompany>,
	projects: HashMap<RecordId, Project>,
}
