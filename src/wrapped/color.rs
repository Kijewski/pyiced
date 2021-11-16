use std::fmt::Display;
use std::num::FpCategory;

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

fn clamp_channel(c: f32) -> PyResult<f32> {
    match c.classify() {
        FpCategory::Nan => Err(PyErr::new::<PyValueError, _>(
            "Color channel value cannot be NaN.",
        )),
        FpCategory::Infinite => match c > 0.0f32 {
            true => Ok(1.0f32),
            false => Ok(0.0f32),
        },
        FpCategory::Zero | FpCategory::Subnormal => Ok(0.0f32),
        FpCategory::Normal => match c {
            c if c >= 1.0f32 => Ok(1.0f32),
            c if c <= 0.0f32 => Ok(0.0f32),
            c => Ok(c),
        },
    }
}

#[pymethods]
impl WrappedColor {
    #[new]
    fn new(r: f32, g: f32, b: f32, a: Option<f32>) -> PyResult<Self> {
        Ok(Self(Color {
            r: clamp_channel(r)?,
            g: clamp_channel(g)?,
            b: clamp_channel(b)?,
            a: match a {
                Some(a) => clamp_channel(a)?,
                None => 1.0f32,
            },
        }))
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

    #[classattr]
    #[allow(non_snake_case)]
    fn __match_args__() -> (&'static str, &'static str, &'static str, &'static str) {
        ("r", "g", "b", "a")
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
