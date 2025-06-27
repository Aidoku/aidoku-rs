//! Parses date strings into Unix timestamps.
use chrono::{Duration, NaiveDate, TimeZone, Utc};

static mut UTC_OFFSET: i64 = -1;

#[link(wasm_import_module = "std")]
extern "C" {
	fn utc_offset() -> i64;
}

unsafe fn get_utc_offset() -> i64 {
	if UTC_OFFSET == -1 {
		UTC_OFFSET = utc_offset();
	}
	UTC_OFFSET
}

pub fn parse_date<T: AsRef<str>, U: AsRef<str>>(date_str: T, format: U) -> Option<i64> {
	NaiveDate::parse_from_str(date_str.as_ref(), format.as_ref())
		.ok()?
		.and_hms_opt(0, 0, 0)
		.map(|d| Utc.from_utc_datetime(&d).timestamp())
}

pub fn parse_local_date<T: AsRef<str>, U: AsRef<str>>(date_str: T, format: U) -> Option<i64> {
	let date = NaiveDate::parse_from_str(date_str.as_ref(), format.as_ref())
		.ok()?
		.and_hms_opt(0, 0, 0)?;
	let offset = unsafe { get_utc_offset() };
	let adjusted = date + Duration::seconds(offset);
	Some(Utc.from_utc_datetime(&adjusted).timestamp())
}
