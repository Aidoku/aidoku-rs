use anyhow::{anyhow, Result};
use wasmer::*;

mod defaults;
mod html;
mod net;
mod store;

pub use defaults::*;
pub use html::*;
pub use net::*;
pub use store::*;

/// A standard descriptor, used for data exchange between the runner and the source (reference id).
///
/// Valid descriptors will always be positive.
pub type Rid = i32;

/// An error code, descriptor, or result value.
///
/// Error codes are negative, while descriptors are positive.
/// A zero value indicates success.
pub type FFIResult = i32;

/// A WebAssembly pointer.
pub type Ptr = u32;

#[derive(Default)]
pub struct WasmEnv {
	pub memory: Option<Memory>,
	pub store: GlobalStore,
	pub defaults: UserDefaults,
	pub stdout: String,
}

impl WasmEnv {
	pub fn new() -> Self {
		Self {
			memory: None,
			store: GlobalStore::new(),
			defaults: UserDefaults::new(),
			stdout: String::new(),
		}
	}

	pub fn read_u32(&self, store: &(impl AsStoreRef + ?Sized), ptr: Ptr) -> Result<u32> {
		let memory = self
			.memory
			.as_ref()
			.ok_or(anyhow!("Memory not initialized"))?;
		let memory_view = memory.view(&store);
		let ptr: WasmPtr<u32> = WasmPtr::new(ptr);
		let value = ptr.read(&memory_view).unwrap_or(0);
		Ok(value)
	}

	pub fn read_values<T: ValueType>(
		&self,
		store: &(impl AsStoreRef + ?Sized),
		ptr: Ptr,
		len: u32,
	) -> Result<Vec<T>> {
		if len == 0 {
			return Ok(Vec::new());
		}
		let memory = self
			.memory
			.as_ref()
			.ok_or(anyhow!("Memory not initialized"))?;
		let memory_view = memory.view(&store);

		let ptr: WasmPtr<T> = WasmPtr::new(ptr);
		let mut data = Vec::new();
		let bytes = ptr.slice(&memory_view, len)?;
		for byte in bytes.iter() {
			data.push(byte.read()?);
		}
		Ok(data)
	}

	pub fn read_bytes(
		&self,
		store: &(impl AsStoreRef + ?Sized),
		ptr: Ptr,
		len: u32,
	) -> Result<Vec<u8>> {
		if len == 0 {
			return Ok(Vec::new());
		}
		let memory = self
			.memory
			.as_ref()
			.ok_or(anyhow!("Memory not initialized"))?;
		let memory_view = memory.view(&store);

		let ptr: WasmPtr<u8> = WasmPtr::new(ptr);
		let mut data = Vec::new();
		let bytes = ptr.slice(&memory_view, len)?;
		for byte in bytes.iter() {
			data.push(byte.read()?);
		}
		Ok(data)
	}

	pub fn read_item_bytes(&self, store: &(impl AsStoreRef + ?Sized), ptr: Ptr) -> Result<Vec<u8>> {
		let len = self.read_u32(store, ptr)?;
		self.read_bytes(store, ptr + 8, len - 8)
	}

	pub fn read_string(
		&self,
		store: &(impl AsStoreRef + ?Sized),
		ptr: Ptr,
		len: u32,
	) -> Result<String> {
		if len == 0 {
			return Ok(String::new());
		}
		let memory = self
			.memory
			.as_ref()
			.ok_or(anyhow!("Memory not initialized"))?;
		let memory_view = memory.view(&store);
		let ptr: WasmPtr<u8> = WasmPtr::new(ptr);
		let str = ptr.read_utf8_string(&memory_view, len)?;
		Ok(str)
	}

	pub fn write_string(
		&self,
		store: &(impl AsStoreRef + ?Sized),
		ptr: Ptr,
		str: &str,
	) -> Result<()> {
		self.write_buffer(store, ptr, str.as_bytes())
	}

	pub fn write_buffer(
		&self,
		store: &(impl AsStoreRef + ?Sized),
		ptr: Ptr,
		bytes: &[u8],
	) -> Result<()> {
		let memory = self
			.memory
			.as_ref()
			.ok_or(anyhow!("Memory not initialized"))?;
		let memory_view = memory.view(&store);
		memory_view.write(ptr as u64, bytes)?;
		Ok(())
	}

	pub fn write_values<T: ValueType>(
		&self,
		store: &(impl AsStoreRef + ?Sized),
		ptr: Ptr,
		values: Vec<T>,
	) -> Result<()> {
		let memory = self
			.memory
			.as_ref()
			.ok_or(anyhow!("Memory not initialized"))?;
		let memory_view = memory.view(&store);

		let ptr: WasmPtr<T> = WasmPtr::new(ptr);
		let buffer = ptr.slice(&memory_view, values.len() as u32)?;
		for (idx, val) in values.into_iter().enumerate() {
			buffer.write(idx as u64, val)?;
		}
		Ok(())
	}

	pub fn write_stdout(&mut self, str: &str) {
		self.stdout.push_str(str);
	}
}
