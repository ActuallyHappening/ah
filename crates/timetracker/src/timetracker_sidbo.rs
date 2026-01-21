use ah_persistence::{packaji_sidbo, sidbo::SidboTcita};

use crate::{
	prelude::*,
	timetracker_ckaji::{BillingCompanyCkaji, ProjectCkaji},
};

packaji_sidbo!(pub struct ProjectSidbo {
	ckaji: ProjectCkaji,
	...
});
// impl Ka_tcita for ProjectSidbo {
// 	const TI_SELTCITA_BAU_LA_LOJBAN: &str = "TODO ah-timetracker ckaji project";
// }

packaji_sidbo!(pub struct BillingCompanySidbo {
	ckaji: BillingCompanyCkaji,
	...
});
// impl Ka_tcita for BillingCompanySidbo {
// 	const TI_SELTCITA_BAU_LA_LOJBAN: &str = "TODO ah-timetracker ckaji billing company";
// }
