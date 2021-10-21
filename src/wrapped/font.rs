use iced::Font;
use pyo3::prelude::*;
use pyo3::PyObjectProtocol;

use crate::common::debug_str;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedFont>()?;
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Private;

#[pyclass(name = "Font", module = "pyiced.pyiced")]
#[derive(Debug, Clone)]
pub(crate) struct WrappedFont(pub Font, Private);

#[pymethods]
impl WrappedFont {
    #[new]
    fn new(_name: &str, _bytes: &[u8]) -> Self {
        todo!() // needs 'static lifetime
    }

    #[classattr]
    #[allow(non_snake_case)]
    fn DEFAULT() -> Self {
        Self(Font::Default, Private)
    }
}

#[pyproto]
impl PyObjectProtocol for WrappedFont {
    fn __str__(&self) -> PyResult<String> {
        debug_str(&self.0)
    }
}
