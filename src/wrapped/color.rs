use std::fmt::Display;

use iced::Color;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use crate::common::debug_str;
use crate::format_to_py;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedColor>()?;
    Ok(())
}

/// Color(r, g, b, a=1.0)
/// --
///
/// A color in the sRGB color space.
///
/// Parameters
/// ----------
/// r : float
///     Red component, 0.0 – 1.0
/// g : float
///     Green component, 0.0 – 1.0
/// b : float
///     Blue component, 0.0 – 1.0
/// a : float
///     Alpha channel, 0.0 – 1.0 (0.0 = transparent; 1.0 = opaque)
#[pyclass(name = "Color", module = "pyiced")]
#[derive(Debug, Clone)]
pub(crate) struct WrappedColor(pub Color);

#[pymethods]
impl WrappedColor {
    #[new]
    fn new(r: f32, g: f32, b: f32, a: Option<f32>) -> PyResult<Self> {
        let a = a.unwrap_or(1.0);
        for v in [r, g, b, a] {
            if !v.is_finite() || v < 0.0 || v > 1.0 {
                return Err(PyErr::new::<PyValueError, _>(
                    "All color channel values need to be inside 0.0 to 1.0 (inclusively).",
                ));
            }
        }
        Ok(Self(Color { r, g, b, a }))
    }

    #[classattr]
    #[allow(non_snake_case)]
    fn BLACK() -> Self {
        Self(Color::BLACK)
    }

    #[classattr]
    #[allow(non_snake_case)]
    fn WHITE() -> Self {
        Self(Color::WHITE)
    }

    #[classattr]
    #[allow(non_snake_case)]
    fn TRANSPARENT() -> Self {
        Self(Color::TRANSPARENT)
    }

    /// Red component, 0.0 – 1.0
    ///
    /// Returns
    /// -------
    /// float
    ///     Color channel value
    #[getter]
    fn r(&self) -> f32 {
        self.0.r
    }

    /// Green component, 0.0 – 1.0
    ///
    /// Returns
    /// -------
    /// float
    ///     Color channel value
    #[getter]
    fn g(&self) -> f32 {
        self.0.g
    }

    /// Blue component, 0.0 – 1.0
    ///
    /// Returns
    /// -------
    /// float
    ///     Color channel value
    #[getter]
    fn b(&self) -> f32 {
        self.0.b
    }

    /// Alpha channel, 0.0 – 1.0 (0.0 = transparent; 1.0 = opaque)
    ///
    /// Returns
    /// -------
    /// float
    ///     Color channel value
    #[getter]
    fn a(&self) -> f32 {
        self.0.a
    }

    fn __str__(&self) -> PyResult<String> {
        debug_str(&self.0)
    }

    fn __repr__(&self) -> PyResult<String> {
        format_to_py!("{}", ColorFormat(&self.0))
    }
}

#[derive(Clone)]
pub(crate) struct ColorFormat<'a>(pub &'a Color);

impl Display for ColorFormat<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let &Color { r, g, b, a } = self.0;
        if a != 1.0 {
            write!(f, "Color({}, {}, {}, a={})", r, g, b, a)
        } else {
            write!(f, "Color({}, {}, {})", r, g, b)
        }
    }
}
