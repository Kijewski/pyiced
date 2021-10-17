use pyo3::{PyObjectProtocol, prelude::*};

use crate::common::debug_str;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedFont>()?;
    Ok(())
}

#[pyclass(name="Font", module="pyiced.pyiced")]
#[derive(Debug, Clone)]
pub(crate) struct WrappedFont(pub iced::Font);

#[pyproto]
impl PyObjectProtocol for WrappedFont {
    fn __str__(&self) -> PyResult<String> {
        debug_str(&self.0)
    }
}
