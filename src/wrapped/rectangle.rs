use std::borrow::Cow;

use iced::{Point, Rectangle, Size};
use pyo3::prelude::*;

use crate::common::debug_str;
use crate::format_to_cow;
use crate::wrapped::{WrappedPoint, WrappedSize};

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedRectangle>()?;
    Ok(())
}

/// Rectangle(top_left, size)
/// --
///
/// A rectangle.
///
/// See also
/// --------
/// `iced::Rectangle <https://docs.rs/iced/0.3.0/iced/struct.Rectangle.html>`_
///
/// Arguments
/// ---------
/// top_left : Point
///     The top-left corner.
/// size : Size
///     The size of the rectangle.
#[pyclass(name = "Rectangle", module = "pyiced")]
#[derive(Debug, Clone)]
pub(crate) struct WrappedRectangle(pub Rectangle);

#[pymethods]
impl WrappedRectangle {
    #[new]
    fn new(top_left: &WrappedPoint, size: &WrappedSize) -> Self {
        Self(Rectangle::new(top_left.0, size.0))
    }

    /// with_size(size)
    /// --
    ///
    /// Creates a new Rectangle with its top-left corner at the origin and with the provided Size.
    ///
    /// Arguments
    /// ---------
    /// size : Size
    ///     Size of the new Rectangle
    ///
    /// Returns
    /// -------
    /// Rectangle
    ///     The new Rectangle.
    #[staticmethod]
    fn with_size(size: &WrappedSize) -> Self {
        Self(Rectangle::with_size(size.0))
    }

    /// X coordinate of the top-left corner.
    ///
    /// Returns
    /// -------
    /// float
    ///     The "top_left.x" parameter given when constructing this point.
    #[getter]
    fn x(&self) -> f32 {
        self.0.x
    }

    /// Y coordinate of the top-left corner.
    ///
    /// Returns
    /// -------
    /// float
    ///     The "top_left.y" parameter given when constructing this point.
    #[getter]
    fn y(&self) -> f32 {
        self.0.y
    }

    /// Width of the rectangle.
    ///
    /// Returns
    /// -------
    /// float
    ///     The "size.width" parameter given when constructing this point.
    #[getter]
    fn width(&self) -> f32 {
        self.0.width
    }

    /// Height of the rectangle.
    ///
    /// Returns
    /// -------
    /// float
    ///     The "size.height" parameter given when constructing this point.
    #[getter]
    fn height(&self) -> f32 {
        self.0.height
    }

    /// The top-left corner.
    ///
    /// Returns
    /// -------
    /// Point
    ///     The "top_left" parameter given when constructing this point.
    #[getter]
    fn top_left(&self) -> WrappedPoint {
        WrappedPoint(Point {
            x: self.0.x,
            y: self.0.y,
        })
    }

    /// The size of the rectangle.
    ///
    /// Returns
    /// -------
    /// Size
    ///     The "size" parameter given when constructing this point.
    #[getter]
    fn size(&self) -> WrappedSize {
        WrappedSize(Size {
            width: self.0.width,
            height: self.0.height,
        })
    }

    #[classattr]
    #[allow(non_snake_case)]
    fn __match_args__() -> (&'static str, &'static str) {
        ("top_left", "size")
    }

    fn __str__(&self) -> PyResult<String> {
        debug_str(&self.0)
    }

    fn __repr__(&self) -> PyResult<Cow<'static, str>> {
        let Rectangle {
            x,
            y,
            width,
            height,
        } = self.0;
        format_to_cow!(
            "Rectangle(Point({:?}, {:?}), Size({:?}, {:?}))",
            x,
            y,
            width,
            height,
        )
    }
}
