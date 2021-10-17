use iced::Point;
use pyo3::prelude::*;

use crate::common::{debug_str, validate_f32};
use crate::format_to_py;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedPoint>()?;
    Ok(())
}

/// Point(x, y)
/// --
///
/// A 2D point.
///
/// Parameters
/// ----------
/// x : float
///     The X coordinate.
/// y : float
///     The Y coordinate.
///
/// See also
/// --------
/// `iced::Point <https://docs.rs/iced/0.3.0/iced/struct.Point.html>`_
#[pyclass(name = "Point", module = "pyiced")]
#[derive(Debug, Clone)]
pub(crate) struct WrappedPoint(pub Point);

#[pymethods]
impl WrappedPoint {
    #[new]
    fn new(x: f32, y: f32) -> PyResult<Self> {
        Ok(Self(Point {
            x: validate_f32(x)?,
            y: validate_f32(y)?,
        }))
    }

    /// The X coordinate.
    ///
    /// Returns
    /// -------
    /// float
    ///     The "x" parameter given when constructing this point.
    #[getter]
    fn x(&self) -> f32 {
        self.0.x
    }

    /// The Y coordinate.
    ///
    /// Returns
    /// -------
    /// float
    ///     The "y" parameter given when constructing this point.
    #[getter]
    fn y(&self) -> f32 {
        self.0.y
    }

    /// The origin (i.e. a Point at (0, 0)).
    #[classattr]
    #[allow(non_snake_case)]
    fn ORIGIN() -> Self {
        Self(Point::ORIGIN)
    }

    /// distance($self, /, to)
    /// --
    ///
    /// Computes the distance to another point.
    ///
    /// Arguments
    /// ---------
    /// to : Point
    ///     The other point.
    fn distance(&self, to: &WrappedPoint) -> f32 {
        self.0.distance(to.0)
    }

    #[classattr]
    #[allow(non_snake_case)]
    fn __match_args__() -> (&'static str, &'static str) {
        ("x", "y")
    }

    fn __str__(&self) -> PyResult<String> {
        debug_str(&self.0)
    }

    fn __repr__(&self) -> PyResult<String> {
        let Point { x, y } = self.0;
        format_to_py!("Point({}, {})", x, y)
    }
}
