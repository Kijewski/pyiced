use iced::Color;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::PyObjectProtocol;

use crate::common::debug_str;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedColor>()?;
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Private;

/// Color(/, r, g, b, a=1.0)
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
pub(crate) struct WrappedColor(pub Color, Private);

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
        Ok(Self(Color { r, g, b, a }, Private))
    }

    #[classattr]
    #[allow(non_snake_case)]
    fn BLACK() -> Self {
        Self(Color::BLACK, Private)
    }

    #[classattr]
    #[allow(non_snake_case)]
    fn WHITE() -> Self {
        Self(Color::WHITE, Private)
    }

    #[classattr]
    #[allow(non_snake_case)]
    fn TRANSPARENT() -> Self {
        Self(Color::TRANSPARENT, Private)
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
}

#[pyproto]
impl PyObjectProtocol for WrappedColor {
    fn __str__(&self) -> PyResult<String> {
        debug_str(&self.0)
    }

    fn __repr__(&self) -> String {
        let Color { r, g, b, a } = self.0;
        if a != 1.0 {
            format!("Color({}, {}, {}, a={})", r, g, b, a)
        } else {
            format!("Color({}, {}, {})", r, g, b)
        }
    }
}
