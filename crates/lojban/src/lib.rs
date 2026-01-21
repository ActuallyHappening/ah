#![allow(dead_code, unused_imports, non_camel_case_types)]

pub mod siho_selylehu;

pub mod jbovlaste;
pub mod lerfu;
pub mod valsi;

pub mod genterfahi;

/// TODO: guarentee no escaping
pub fn quote(str: impl std::borrow::Borrow<str>) -> String {
	format!("lu {} li'u", str.borrow())
}
