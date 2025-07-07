use crate::{FFIResult, Ptr, Rid, WasmEnv};
use wasmer::FunctionEnvMut;

enum Result {
	Success,
	InvalidDescriptor,
	// InvalidBufferSize,
	FailedMemoryWrite,
	InvalidString,
	InvalidDateString,
}

impl From<Result> for i32 {
	fn from(result: Result) -> i32 {
		match result {
			Result::Success => 0,
			Result::InvalidDescriptor => -1,
			// Result::InvalidBufferSize => -2,
			Result::FailedMemoryWrite => -3,
			Result::InvalidString => -4,
			Result::InvalidDateString => -5,
		}
	}
}

pub fn destroy(mut env: FunctionEnvMut<WasmEnv>, rid: Rid) {
	env.data_mut().store.remove(rid);
}

pub fn buffer_len(env: FunctionEnvMut<WasmEnv>, rid: Rid) -> FFIResult {
	let Some(data) = env.data().store.get(rid).and_then(|item| item.as_encoded()) else {
		return Result::InvalidDescriptor.into();
	};
	data.len() as FFIResult
}

pub fn read_buffer(env: FunctionEnvMut<WasmEnv>, rid: Rid, ptr: Ptr, size: u32) -> FFIResult {
	let Some(data) = env.data().store.get(rid).and_then(|item| item.as_encoded()) else {
		return Result::InvalidDescriptor.into();
	};
	if size as usize <= data.len() {
		let data = data.iter().take(size as usize).copied().collect::<Vec<_>>();
		if env.data().write_buffer(&env, ptr, &data).is_err() {
			Result::FailedMemoryWrite.into()
		} else {
			Result::Success.into()
		}
	} else {
		Result::FailedMemoryWrite.into()
	}
}

pub fn current_date(_env: FunctionEnvMut<WasmEnv>) -> f64 {
	use chrono::Utc;
	Utc::now().timestamp() as f64
}

pub fn utc_offset(_env: FunctionEnvMut<WasmEnv>) -> i64 {
	use chrono::Local;
	Local::now().offset().utc_minus_local() as i64
}

pub fn parse_date(
	env: FunctionEnvMut<WasmEnv>,
	date_str: Ptr,
	date_len: u32,
	format_str: Ptr,
	format_len: u32,
	_locale_str: Ptr,
	_locale_len: u32,
	timezone_str: Ptr,
	timezone_len: u32,
) -> f64 {
	let Ok(string) = env.data().read_string(&env, date_str, date_len) else {
		return Into::<i32>::into(Result::InvalidString) as f64;
	};
	let Ok(format) = env.data().read_string(&env, format_str, format_len) else {
		return Into::<i32>::into(Result::InvalidString) as f64;
	};
	let timezone_string = if timezone_len > 0 {
		env.data()
			.read_string(&env, timezone_str, timezone_len)
			.ok()
	} else {
		None
	};

	let timezone: chrono_tz::Tz = timezone_string
		.as_deref()
		.and_then(|tz_str| tz_str.parse().ok())
		.unwrap_or(chrono_tz::UTC);

	use chrono::NaiveDateTime;

	let Some(timestamp) =
		NaiveDateTime::parse_from_str(&string, &swift_date_format_to_chrono(&format))
			.ok()
			.and_then(|dt| dt.and_local_timezone(timezone).single())
			.map(|dt| dt.timestamp() as f64)
	else {
		return Into::<i32>::into(Result::InvalidDateString) as f64;
	};
	timestamp
}

fn swift_date_format_to_chrono(format: &str) -> String {
	let mut result = String::new();
	let mut chars = format.chars().peekable();

	while let Some(c) = chars.next() {
		let token = match c {
			// year
			'y' => {
				// yyyy or yy
				let mut count = 1;
				while chars.peek() == Some(&'y') {
					chars.next();
					count += 1;
				}
				if count == 2 {
					"%y"
				} else {
					"%Y"
				}
			}
			// month
			'M' => {
				// MM or MMM or MMMM
				let mut count = 1;
				while chars.peek() == Some(&'M') {
					chars.next();
					count += 1;
				}
				match count {
					1 | 2 => "%m",
					3 => "%b",
					_ => "%B",
				}
			}
			// day
			'd' => {
				// dd or d
				while chars.peek() == Some(&'d') {
					chars.next();
				}
				"%d"
			}
			// hour
			'H' => {
				// HH or H
				while chars.peek() == Some(&'H') {
					chars.next();
				}
				"%H"
			}
			'h' => {
				// hh or h
				while chars.peek() == Some(&'h') {
					chars.next();
				}
				"%I"
			}
			// minute
			'm' => {
				// mm or m
				while chars.peek() == Some(&'m') {
					chars.next();
				}
				"%M"
			}
			// second
			's' => {
				// ss or s
				while chars.peek() == Some(&'s') {
					chars.next();
				}
				"%S"
			}
			'a' => "%p", // AM/PM
			// weekday
			'E' => {
				// EEE or EEEE
				let mut count = 1;
				while chars.peek() == Some(&'E') {
					chars.next();
					count += 1;
				}
				if count >= 4 {
					"%A"
				} else {
					"%a"
				}
			}
			// timezone
			'z' => {
				let mut count = 1;
				while chars.peek() == Some(&c) {
					chars.next();
					count += 1;
				}
				if count >= 4 {
					"%Z"
				} else {
					"%z"
				}
			}
			'Z' => {
				while chars.peek() == Some(&c) {
					chars.next();
				}
				"%Z"
			}
			// literal
			c => {
				result.push(c);
				continue;
			}
		};
		result.push_str(token);
	}
	result
}
