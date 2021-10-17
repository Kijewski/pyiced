use pyo3::{PyObjectProtocol, prelude::*};

use crate::common::debug_str;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedColor>()?;
    Ok(())
}

#[pyclass(name="Color", module="pyiced.pyiced")]
#[derive(Debug, Clone)]
pub(crate) struct WrappedColor(pub iced::Color);

#[pymethods]
impl WrappedColor {
    #[new]
    fn new(r: f32, g: f32, b: f32, a: Option<f32>) -> Self {
        let a = a.unwrap_or(1.0);
        let v = iced::Color { r, g, b, a };
        Self(v)
    }
}

#[pyproto]
impl PyObjectProtocol for WrappedColor {
    fn __str__(&self) -> PyResult<String> {
        debug_str(&self.0)
    }
}
