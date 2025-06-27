//! Structs and enums used for drawing on a canvas.
//!
//! A lot of APIs are largely based on [raqote](https://crates.io/crates/raqote).
use crate::alloc::Vec;
use serde::{Deserialize, Serialize};

pub type Transform = euclid::default::Transform2D<f32>;
pub type Angle = euclid::Angle<f32>;

/// A rectangle.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Rect {
	pub x: f32,
	pub y: f32,
	pub width: f32,
	pub height: f32,
}

impl Rect {
	/// Creates a new rectangle with the given position and size.
	pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
		Rect {
			x,
			y,
			width,
			height,
		}
	}
}

/// A 2D point.
#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct Point {
	pub x: f32,
	pub y: f32,
}

impl Point {
	/// Creates a new point with the given coordinates.
	pub fn new(x: f32, y: f32) -> Self {
		Point { x, y }
	}
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum PathOp {
	MoveTo(Point),
	LineTo(Point),
	QuadTo(Point, Point),
	CubicTo(Point, Point, Point),
	Arc(Point, f32, f32, f32),
	Close,
}

/// A 2D path.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Path {
	pub ops: Vec<PathOp>,
	#[serde(skip)]
	#[cfg(feature = "imports")]
	pub(crate) ptr: Option<i32>,
}

impl Path {
	pub fn rect(rect: &Rect) -> Self {
		PathBuilder::new().rect(rect).build()
	}
}

impl PartialEq for Path {
	fn eq(&self, other: &Self) -> bool {
		self.ops == other.ops
	}
}

/// A builder for paths.
#[derive(Debug, Default)]
pub struct PathBuilder {
	path: Path,
}

impl PathBuilder {
	/// Creates a new path builder.
	pub fn new() -> Self {
		PathBuilder::default()
	}

	/// Moves the current point to `x`, `y`.
	pub fn move_to(mut self, x: f32, y: f32) -> Self {
		self.path.ops.push(PathOp::MoveTo(Point::new(x, y)));
		self
	}

	/// Adds a line segment from the current point to `x`, `y`.
	pub fn line_to(mut self, x: f32, y: f32) -> Self {
		self.path.ops.push(PathOp::LineTo(Point::new(x, y)));
		self
	}

	/// Adds a quadratic bezier from the current point to `x`, `y`,
	/// using a control point of `cx`, `cy`.
	pub fn quad_to(mut self, cx: f32, cy: f32, x: f32, y: f32) -> Self {
		self.path
			.ops
			.push(PathOp::QuadTo(Point::new(cx, cy), Point::new(x, y)));
		self
	}

	/// Adds a rect to the path.
	pub fn rect(self, rect: &Rect) -> Self {
		self.move_to(rect.x, rect.y)
			.line_to(rect.x + rect.width, rect.y)
			.line_to(rect.x + rect.width, rect.y + rect.height)
			.line_to(rect.x, rect.y + rect.height)
			.close()
	}

	/// Adds a cubic bezier from the current point to `x`, `y`,
	/// using control points `cx1`, `cy1` and `cx2`, `cy2`.
	pub fn cubic_to(mut self, x: f32, y: f32, cx1: f32, cy1: f32, cx2: f32, cy2: f32) -> Self {
		self.path.ops.push(PathOp::CubicTo(
			Point::new(x, y),
			Point::new(cx1, cy1),
			Point::new(cx2, cy2),
		));
		self
	}

	/// Adds an arc approximated by quadratic beziers with center `x`, `y`
	/// and radius `r` starting at `start_angle` and sweeping by `sweep_angle`.
	/// For a positive `sweep_angle` the sweep is done clockwise, for a negative
	/// `sweep_angle` the sweep is done counterclockwise.
	pub fn arc(mut self, x: f32, y: f32, radius: f32, start_angle: f32, sweep_angle: f32) -> Self {
		self.path.ops.push(PathOp::Arc(
			Point::new(x, y),
			radius,
			start_angle,
			sweep_angle,
		));
		self
	}

	/// Closes the current subpath.
	pub fn close(mut self) -> Self {
		self.path.ops.push(PathOp::Close);
		self
	}

	/// Builds the path.
	pub fn build(self) -> Path {
		#[cfg(feature = "imports")]
		{
			use crate::imports::std::encode;
			let mut path = self.path;
			path.ptr = Some(unsafe { encode(&path) });
			path
		}
		#[cfg(not(feature = "imports"))]
		self.path
	}
}

/// A color with red, green, blue, and alpha components.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Color {
	pub red: f32,
	pub green: f32,
	pub blue: f32,
	pub alpha: f32,
}

impl Color {
	/// Creates a new color with the given components.
	pub fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
		Self {
			red,
			green,
			blue,
			alpha,
		}
	}

	/// The color black.
	pub fn black() -> Self {
		Self::new(0.0, 0.0, 0.0, 1.0)
	}

	/// The color white.
	pub fn white() -> Self {
		Self::new(255.0, 255.0, 255.0, 1.0)
	}

	/// The color red.
	pub fn red() -> Self {
		Self::new(255.0, 0.0, 0.0, 1.0)
	}

	/// The color green.
	pub fn green() -> Self {
		Self::new(0.0, 255.0, 0.0, 1.0)
	}

	/// The color blue.
	pub fn blue() -> Self {
		Self::new(0.0, 0.0, 255.0, 1.0)
	}
}

impl Default for Color {
	fn default() -> Self {
		Self::black()
	}
}

/// The endpoint style of a line.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum LineCap {
	Round,
	Square,
	#[default]
	Butt,
}

/// The style of connected line joins.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum LineJoin {
	Round,
	Bevel,
	#[default]
	Miter,
}

/// The configuration options for drawing strokes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StrokeStyle {
	pub color: Color,
	pub width: f32,
	pub cap: LineCap,
	pub join: LineJoin,
	pub miter_limit: f32,
	pub dash_array: Vec<f32>,
	pub dash_offset: f32,
}

impl Default for StrokeStyle {
	fn default() -> Self {
		Self {
			color: Default::default(),
			width: 1.,
			cap: Default::default(),
			join: Default::default(),
			miter_limit: 10.,
			dash_array: Vec::new(),
			dash_offset: 0.,
		}
	}
}

/// A standard typeface weight.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FontWeight {
	UltraLight = 0,
	Thin = 1,
	Light = 2,
	#[default]
	Regular = 3,
	Medium = 4,
	Semibold = 5,
	Bold = 6,
	Heavy = 7,
	Black = 8,
}

impl From<u8> for FontWeight {
	fn from(value: u8) -> Self {
		match value {
			0 => FontWeight::UltraLight,
			1 => FontWeight::Thin,
			2 => FontWeight::Light,
			3 => FontWeight::Regular,
			4 => FontWeight::Medium,
			5 => FontWeight::Semibold,
			6 => FontWeight::Bold,
			7 => FontWeight::Heavy,
			8 => FontWeight::Black,
			_ => FontWeight::Regular,
		}
	}
}
