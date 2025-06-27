//! Module for working with bitmap canvases.
use super::{std::destroy, FFIResult, Ptr, Rid};
use crate::alloc::Vec;
use crate::imports::std::{encode, free_result, read_buffer};
use serde::{Deserialize, Serialize};

pub use crate::canvas::*;

#[link(wasm_import_module = "canvas")]
extern "C" {
	fn new_context(width: f32, height: f32) -> Rid;

	fn set_transform(
		context: Rid,
		translate_x: f32,
		translate_y: f32,
		scale_x: f32,
		scale_y: f32,
		rotate_angle: f32,
	) -> FFIResult;
	fn copy_image(
		context: Rid,
		image: Rid,
		src_x: f32,
		src_y: f32,
		src_width: f32,
		src_height: f32,
		dst_x: f32,
		dst_y: f32,
		dst_width: f32,
		dst_height: f32,
	) -> FFIResult;
	fn draw_image(
		context: Rid,
		image: Rid,
		dst_x: f32,
		dst_y: f32,
		dst_width: f32,
		dst_height: f32,
	) -> FFIResult;
	fn fill(context: Rid, path: Ptr, r: f32, g: f32, b: f32, a: f32) -> FFIResult;
	fn stroke(context: Rid, path: Ptr, style: Ptr) -> FFIResult;
	fn draw_text(
		context: Rid,
		text: *const u8,
		text_len: usize,
		size: f32,
		x: f32,
		y: f32,
		font: Rid,
		r: f32,
		g: f32,
		b: f32,
		a: f32,
	) -> FFIResult;
	fn get_image(context: Rid) -> Rid;

	fn new_font(name_ptr: *const u8, name_len: usize) -> FFIResult;
	fn system_font(weight: u8) -> Rid;
	fn load_font(url_ptr: *const u8, url_len: usize) -> FFIResult;

	fn new_image(data_ptr: *const u8, data_len: usize) -> FFIResult;
	fn get_image_data(image_rid: Rid) -> FFIResult;
	fn get_image_width(image_rid: Rid) -> f32;
	fn get_image_height(image_rid: Rid) -> f32;
}

/// Error type for canvas operations.
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum CanvasError {
	InvalidContext,
	InvalidImagePointer,
	InvalidImage,
	InvalidSrcRect,
	InvalidResult,
	InvalidBounds,
	InvalidPath,
	InvalidStyle,
	InvalidString,
	InvalidFont,
	FontLoadFailed,
}

impl CanvasError {
	fn from(value: FFIResult) -> Option<Self> {
		match value {
			-1 => Some(Self::InvalidContext),
			-2 => Some(Self::InvalidImagePointer),
			-3 => Some(Self::InvalidImage),
			-4 => Some(Self::InvalidSrcRect),
			-5 => Some(Self::InvalidResult),
			-6 => Some(Self::InvalidBounds),
			-7 => Some(Self::InvalidPath),
			-8 => Some(Self::InvalidStyle),
			-9 => Some(Self::InvalidString),
			-10 => Some(Self::InvalidFont),
			-11 => Some(Self::FontLoadFailed),
			_ => None,
		}
	}
}

/// A reference to an image.
#[derive(Debug)]
pub struct ImageRef {
	/// The reference id of the stored image.
	///
	/// This property is exposed for the functions that the [register_source](crate::register_source)
	/// macro generates and should not be used directly.
	pub rid: Rid,
	/// Whether the image is externally managed.
	///
	/// This property is exposed for the functions that the [register_source](crate::register_source)
	/// macro generates and should not be used directly.
	pub externally_managed: bool,
}

impl ImageRef {
	pub(crate) fn from(rid: Rid, externally_managed: bool) -> Self {
		ImageRef {
			rid,
			externally_managed,
		}
	}

	pub fn new(data: &[u8]) -> Self {
		let rid = unsafe { new_image(data.as_ptr(), data.len()) };
		ImageRef::from(rid, false)
	}

	pub fn data(&self) -> Vec<u8> {
		let result = unsafe { get_image_data(self.rid) };
		if CanvasError::from(result).is_some() {
			return Vec::new();
		}
		read_buffer(result).unwrap_or_default()
	}

	pub fn width(&self) -> f32 {
		unsafe { get_image_width(self.rid) }
	}

	pub fn height(&self) -> f32 {
		unsafe { get_image_height(self.rid) }
	}
}

impl PartialEq for ImageRef {
	fn eq(&self, other: &Self) -> bool {
		self.rid == other.rid
	}
}

impl Drop for ImageRef {
	fn drop(&mut self) {
		if !self.externally_managed {
			unsafe { destroy(self.rid) }
		}
	}
}

impl Serialize for ImageRef {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		self.rid.serialize(serializer)
	}
}

impl<'de> Deserialize<'de> for ImageRef {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		// when deserializing from a response struct, the image ref should be managed externally
		Rid::deserialize(deserializer).map(|rid| ImageRef::from(rid, true))
	}
}

impl Drop for Path {
	fn drop(&mut self) {
		if let Some(ptr) = self.ptr {
			unsafe { free_result(ptr) };
		}
	}
}

/// A reference to a font.
#[derive(Debug)]
pub struct Font {
	rid: Rid,
}

impl Font {
	/// Creates a new font with the given family.
	pub fn new(font_family: &str) -> Result<Self, CanvasError> {
		let rid = unsafe { new_font(font_family.as_ptr(), font_family.len()) };
		if let Some(err) = CanvasError::from(rid) {
			return Err(err);
		}
		Ok(Self { rid })
	}

	/// Creates a new font with the system default family and the given weight.
	pub fn system(weight: FontWeight) -> Self {
		let rid = unsafe { system_font(weight as u8) };
		Self { rid }
	}

	/// Loads a font from the given URL.
	pub fn load(url: &str) -> Result<Self, CanvasError> {
		let rid = unsafe { load_font(url.as_ptr(), url.len()) };
		if let Some(err) = CanvasError::from(rid) {
			return Err(err);
		}
		Ok(Self { rid })
	}
}

/// A canvas for drawing images.
#[derive(Debug)]
pub struct Canvas {
	rid: Rid,
}

impl Canvas {
	/// Creates a new canvas with the given size.
	pub fn new(width: f32, height: f32) -> Self {
		let rid = unsafe { new_context(width, height) };
		Canvas { rid }
	}

	/// Sets the transformation matrix for the canvas.
	pub fn set_transform(&mut self, transform: &Transform) {
		use num_traits::float::Float;

		// convert transform matrix to translate, scale, and rotate values
		let scale_x = (transform.m11 * transform.m11 + transform.m12 * transform.m12).sqrt();
		let scale_y = (transform.m21 * transform.m21 + transform.m22 * transform.m22).sqrt();
		let rotate_angle = transform.m12.atan2(transform.m11);
		let translate_x = (transform.m31 * Float::cos(rotate_angle)
			+ transform.m32 * Float::sin(rotate_angle))
			/ scale_x;
		let translate_y = (transform.m32 * Float::cos(rotate_angle)
			- transform.m31 * Float::sin(rotate_angle))
			/ scale_y;

		unsafe {
			set_transform(
				self.rid,
				translate_x,
				translate_y,
				scale_x,
				scale_y,
				rotate_angle,
			);
		}
	}

	/// Draws an image onto the canvas.
	pub fn draw_image(&mut self, image: &ImageRef, dst_rect: Rect) {
		unsafe {
			draw_image(
				self.rid,
				image.rid,
				dst_rect.x,
				dst_rect.y,
				dst_rect.width,
				dst_rect.height,
			);
		}
	}

	/// Copies an area of the canvas onto another area.
	pub fn copy_image(&mut self, image: &ImageRef, src_rect: Rect, dst_rect: Rect) {
		unsafe {
			copy_image(
				self.rid,
				image.rid,
				src_rect.x,
				src_rect.y,
				src_rect.width,
				src_rect.height,
				dst_rect.x,
				dst_rect.y,
				dst_rect.width,
				dst_rect.height,
			);
		}
	}

	/// Fills a path with a given color.
	pub fn fill(&mut self, path: &Path, color: &Color) {
		let Some(path_ptr) = path.ptr else { return };
		unsafe {
			fill(
				self.rid,
				path_ptr,
				color.red,
				color.green,
				color.blue,
				color.alpha,
			);
		}
	}

	/// Strokes a path with a given style.
	pub fn stroke(&mut self, path: &Path, style: &StrokeStyle) {
		let Some(path_ptr) = path.ptr else { return };
		let style_ptr = unsafe { encode(style) };
		unsafe {
			stroke(self.rid, path_ptr, style_ptr);
		}
		unsafe {
			free_result(style_ptr);
		}
	}

	/// Draws text onto the canvas.
	pub fn draw_text(&mut self, text: &str, size: f32, pos: &Point, font: &Font, color: &Color) {
		unsafe {
			draw_text(
				self.rid,
				text.as_ptr(),
				text.len(),
				size,
				pos.x,
				pos.y,
				font.rid,
				color.red,
				color.green,
				color.blue,
				color.alpha,
			);
		}
	}

	/// Converts the internal canvas into an image.
	pub fn get_image(self) -> ImageRef {
		ImageRef::from(unsafe { get_image(self.rid) }, false)
	}
}

impl Drop for Canvas {
	fn drop(&mut self) {
		unsafe { destroy(self.rid) }
	}
}
