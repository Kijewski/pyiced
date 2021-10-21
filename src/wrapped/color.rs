use iced::Color;
use pyo3::prelude::*;
use pyo3::PyObjectProtocol;

use crate::common::debug_str;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedColor>()?;
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Private;

#[pyclass(name = "Color", module = "pyiced.pyiced")]
#[derive(Debug, Clone)]
pub(crate) struct WrappedColor(pub Color, Private);

#[pymethods]
impl WrappedColor {
    #[new]
    fn new(r: f32, g: f32, b: f32, a: Option<f32>) -> Self {
        let a = a.unwrap_or(1.0);
        let v = Color { r, g, b, a };
        Self(v, Private)
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
}

#[pyproto]
impl PyObjectProtocol for WrappedColor {
    fn __str__(&self) -> PyResult<String> {
        debug_str(&self.0)
    }
}
