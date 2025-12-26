use std::io::Cursor;

use crate::{
	FFIResult, Ptr, Rid, WasmEnv,
	libs::{ImageData, StoreItem},
};
use aidoku::canvas::{FontWeight, PathOp};
use euclid::Angle;
use font_kit::{
	family_name::FamilyName,
	properties::{Properties, Weight},
	source::SystemSource,
};
use image::{ImageBuffer, ImageReader};
use raqote::{DrawOptions, DrawTarget, LineCap, LineJoin, Point, Source, Transform};
use wasmer::FunctionEnvMut;

enum Result {
	Success,
	InvalidContext,
	InvalidImagePointer,
	InvalidImage,
	// InvalidSrcRec,
	// InvalidResult,
	// InvalidBounds,
	InvalidPath,
	InvalidStyle,
	InvalidString,
	InvalidFont,
	InvalidData,
	FontLoadFailed,
}

impl From<Result> for i32 {
	fn from(result: Result) -> i32 {
		match result {
			Result::Success => 0,
			Result::InvalidContext => -1,
			Result::InvalidImagePointer => -2,
			Result::InvalidImage => -3,
			// Result::InvalidSrcRec => -4,
			// Result::InvalidResult => -5,
			// Result::InvalidBounds => -6,
			Result::InvalidPath => -7,
			Result::InvalidStyle => -8,
			Result::InvalidString => -9,
			Result::InvalidFont => -10,
			Result::InvalidData => -11,
			Result::FontLoadFailed => -12,
		}
	}
}

fn path_to_raqote_path(path: aidoku::canvas::Path) -> raqote::Path {
	let mut result = raqote::PathBuilder::new();
	for op in path.ops.iter() {
		match op {
			PathOp::MoveTo(point) => result.move_to(point.x, point.y),
			PathOp::LineTo(point) => result.line_to(point.x, point.y),
			PathOp::QuadTo(to, control) => result.quad_to(control.x, control.y, to.x, to.y),
			PathOp::CubicTo(to, c1, c2) => result.cubic_to(c1.x, c1.y, c2.x, c2.y, to.x, to.y),
			PathOp::Arc(to, radius, start, sweep) => {
				result.arc(to.x, to.y, *radius, *start, *sweep)
			}
			PathOp::Close => result.close(),
		}
	}
	result.finish()
}

pub fn new_context(mut env: FunctionEnvMut<WasmEnv>, width: f32, height: f32) -> Rid {
	let canvas = DrawTarget::new(width as i32, height as i32);
	env.data_mut()
		.store
		.store(StoreItem::Canvas(Box::new(canvas)))
}
pub fn set_transform(
	mut env: FunctionEnvMut<WasmEnv>,
	context: Rid,
	translate_x: f32,
	translate_y: f32,
	scale_x: f32,
	scale_y: f32,
	rotate_angle: f32,
) -> FFIResult {
	let Some(canvas) = env
		.data_mut()
		.store
		.get_mut(context)
		.and_then(|item| item.as_canvas())
	else {
		return Result::InvalidContext.into();
	};
	canvas.set_transform(
		&Transform::translation(translate_x, translate_y)
			.then_scale(scale_x, scale_y)
			.then_rotate(Angle {
				radians: rotate_angle,
			}),
	);
	Result::Success.into()
}
#[allow(clippy::too_many_arguments)]
pub fn copy_image(
	_env: FunctionEnvMut<WasmEnv>,
	_context: Rid,
	_image: Rid,
	_src_x: f32,
	_src_y: f32,
	_src_width: f32,
	_src_height: f32,
	_dst_x: f32,
	_dst_y: f32,
	_dst_width: f32,
	_dst_height: f32,
) -> FFIResult {
	-1
}
pub fn draw_image(
	_env: FunctionEnvMut<WasmEnv>,
	_context: Rid,
	_image: Rid,
	_dst_x: f32,
	_dst_y: f32,
	_dst_width: f32,
	_dst_height: f32,
) -> FFIResult {
	-1
}
pub fn fill(
	mut env: FunctionEnvMut<WasmEnv>,
	context: Rid,
	path_ptr: Ptr,
	r: f32,
	g: f32,
	b: f32,
	a: f32,
) -> FFIResult {
	let Some(path): Option<aidoku::canvas::Path> = env
		.data()
		.read_item_bytes(&env, path_ptr)
		.ok()
		.and_then(|data| postcard::from_bytes(&data).ok())
	else {
		return Result::InvalidPath.into();
	};
	let Some(canvas) = env
		.data_mut()
		.store
		.get_mut(context)
		.and_then(|item| item.as_canvas())
	else {
		return Result::InvalidContext.into();
	};
	let final_path = path_to_raqote_path(path);
	canvas.fill(
		&final_path,
		&Source::Solid(raqote::SolidSource {
			r: r as u8,
			g: g as u8,
			b: b as u8,
			a: (a * 255.0) as u8,
		}),
		&DrawOptions::default(),
	);
	Result::Success.into()
}
pub fn stroke(
	mut env: FunctionEnvMut<WasmEnv>,
	context: Rid,
	path_ptr: Ptr,
	style_ptr: Ptr,
) -> FFIResult {
	let Some(path): Option<aidoku::canvas::Path> = env
		.data()
		.read_item_bytes(&env, path_ptr)
		.ok()
		.and_then(|data| postcard::from_bytes(&data).ok())
	else {
		return Result::InvalidPath.into();
	};
	let Some(style): Option<aidoku::canvas::StrokeStyle> = env
		.data()
		.read_item_bytes(&env, style_ptr)
		.ok()
		.and_then(|data| postcard::from_bytes(&data).ok())
	else {
		return Result::InvalidStyle.into();
	};
	let Some(canvas) = env
		.data_mut()
		.store
		.get_mut(context)
		.and_then(|item| item.as_canvas())
	else {
		return Result::InvalidContext.into();
	};
	let final_path = path_to_raqote_path(path);
	canvas.stroke(
		&final_path,
		&Source::Solid(raqote::SolidSource {
			r: style.color.red as u8,
			g: style.color.green as u8,
			b: style.color.blue as u8,
			a: (style.color.alpha * 255.0) as u8,
		}),
		&raqote::StrokeStyle {
			width: style.width,
			cap: match style.cap {
				aidoku::canvas::LineCap::Butt => LineCap::Butt,
				aidoku::canvas::LineCap::Round => LineCap::Round,
				aidoku::canvas::LineCap::Square => LineCap::Square,
			},
			join: match style.join {
				aidoku::canvas::LineJoin::Miter => LineJoin::Miter,
				aidoku::canvas::LineJoin::Round => LineJoin::Round,
				aidoku::canvas::LineJoin::Bevel => LineJoin::Bevel,
			},
			miter_limit: style.miter_limit,
			dash_array: style.dash_array,
			dash_offset: style.dash_offset,
		},
		&DrawOptions::default(),
	);
	Result::Success.into()
}
#[allow(clippy::too_many_arguments)]
pub fn draw_text(
	mut env: FunctionEnvMut<WasmEnv>,
	context: Rid,
	text_ptr: Ptr,
	text_len: u32,
	size: f32,
	x: f32,
	y: f32,
	font: Rid,
	r: f32,
	g: f32,
	b: f32,
	a: f32,
) -> FFIResult {
	let Ok(text) = env.data().read_string(&env, text_ptr, text_len) else {
		return Result::InvalidString.into();
	};
	let Some(font) = env
		.data()
		.store
		.get(font)
		.and_then(|item| item.as_font())
		.cloned()
	else {
		return Result::InvalidFont.into();
	};
	let Some(canvas) = env
		.data_mut()
		.store
		.get_mut(context)
		.and_then(|item| item.as_canvas())
	else {
		return Result::InvalidContext.into();
	};
	canvas.draw_text(
		&font,
		size,
		&text,
		Point::new(x, y),
		&Source::Solid(raqote::SolidSource {
			r: r as u8,
			g: g as u8,
			b: b as u8,
			a: (a * 255.0) as u8,
		}),
		&DrawOptions::default(),
	);
	Result::Success.into()
}
pub fn get_image(mut env: FunctionEnvMut<WasmEnv>, context: Rid) -> Rid {
	let Some(canvas) = env
		.data_mut()
		.store
		.get_mut(context)
		.and_then(|item| item.as_canvas())
	else {
		return Result::InvalidContext.into();
	};
	let data = canvas.get_data_u8().to_vec();
	let image = ImageData {
		data,
		width: canvas.width(),
		height: canvas.height(),
	};
	env.data_mut().store.store(StoreItem::ImageData(image))
}

pub fn new_font(mut env: FunctionEnvMut<WasmEnv>, name_ptr: Ptr, name_len: u32) -> FFIResult {
	let Ok(name) = env.data().read_string(&env, name_ptr, name_len) else {
		return Result::InvalidString.into();
	};
	let Some(font) = SystemSource::new()
		.select_best_match(&[FamilyName::Title(name)], &Properties::new())
		.ok()
		.and_then(|h| h.load().ok())
	else {
		return Result::InvalidFont.into();
	};
	env.data_mut().store.store(StoreItem::Font(font))
}
pub fn system_font(mut env: FunctionEnvMut<WasmEnv>, weight: u8) -> Rid {
	let weight = match aidoku::canvas::FontWeight::from(weight) {
		FontWeight::UltraLight => Weight::EXTRA_LIGHT,
		FontWeight::Thin => Weight::THIN,
		FontWeight::Light => Weight::LIGHT,
		FontWeight::Regular => Weight::NORMAL,
		FontWeight::Medium => Weight::MEDIUM,
		FontWeight::Semibold => Weight::SEMIBOLD,
		FontWeight::Bold => Weight::BOLD,
		FontWeight::Heavy => Weight::EXTRA_BOLD,
		FontWeight::Black => Weight::BLACK,
	};
	let Some(font) = SystemSource::new()
		.select_best_match(&[FamilyName::SansSerif], Properties::new().weight(weight))
		.ok()
		.and_then(|h| h.load().ok())
	else {
		return Result::InvalidFont.into();
	};
	env.data_mut().store.store(StoreItem::Font(font))
}
pub fn load_font(_env: FunctionEnvMut<WasmEnv>, _url_ptr: Ptr, _url_len: u32) -> FFIResult {
	// let Ok(url) = env.data().read_string(&env, url_ptr, url_len) else {
	// 	return Result::InvalidString.into();
	// };
	Result::FontLoadFailed.into()
}

pub fn new_image(mut env: FunctionEnvMut<WasmEnv>, data_ptr: Ptr, data_len: u32) -> FFIResult {
	let Ok(data) = env.data().read_bytes(&env, data_ptr, data_len) else {
		return Result::InvalidData.into();
	};
	let cursor = Cursor::new(data);
	let Some(rgba_img) = ImageReader::new(cursor)
		.with_guessed_format()
		.ok()
		.and_then(|r| r.decode().ok())
		.map(|img| img.to_rgb8())
	else {
		return Result::InvalidData.into();
	};
	let width = rgba_img.width() as i32;
	let height = rgba_img.height() as i32;
	let data = rgba_img.into_raw();
	let image = ImageData {
		data,
		width,
		height,
	};
	env.data_mut().store.store(StoreItem::ImageData(image))
}
pub fn get_image_data(mut env: FunctionEnvMut<WasmEnv>, image_rid: Rid) -> FFIResult {
	let Some(image) = env
		.data()
		.store
		.get(image_rid)
		.and_then(|item| item.as_image_data())
	else {
		return Result::InvalidImagePointer.into();
	};
	let Some(img): Option<ImageBuffer<image::Rgba<u8>, _>> =
		ImageBuffer::from_raw(image.width as u32, image.height as u32, image.data.clone())
	else {
		return Result::InvalidImage.into();
	};
	let mut png_data = Vec::new();
	if img
		.write_to(&mut Cursor::new(&mut png_data), image::ImageFormat::Png)
		.is_err()
	{
		return Result::InvalidImage.into();
	}
	env.data_mut().store.store(StoreItem::Encoded(png_data))
}
pub fn get_image_width(env: FunctionEnvMut<WasmEnv>, image_rid: Rid) -> f32 {
	let Some(image) = env
		.data()
		.store
		.get(image_rid)
		.and_then(|item| item.as_image_data())
	else {
		return Into::<i32>::into(Result::InvalidImagePointer) as f32;
	};
	image.width as f32
}
pub fn get_image_height(env: FunctionEnvMut<WasmEnv>, image_rid: Rid) -> f32 {
	let Some(image) = env
		.data()
		.store
		.get(image_rid)
		.and_then(|item| item.as_image_data())
	else {
		return Into::<i32>::into(Result::InvalidImagePointer) as f32;
	};
	image.height as f32
}
