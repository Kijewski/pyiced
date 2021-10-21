use std::path::PathBuf;

use iced::image::Handle;
use pyo3::prelude::*;
use pyo3::PyObjectProtocol;

use crate::common::debug_str;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedImageHandle>()?;
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Private;

#[pyclass(name = "ImageHandle", module = "pyiced")]
#[derive(Debug, Clone)]
pub(crate) struct WrappedImageHandle(pub Handle, Private);

#[pymethods]
impl WrappedImageHandle {
    #[staticmethod]
    fn from_path(path: PathBuf) -> Self {
        Self(Handle::from_path(path), Private)
    }

    #[staticmethod]
    fn from_memory(path: Vec<u8>) -> Self {
        Self(Handle::from_memory(path), Private)
    }
}

#[pyproto]
impl PyObjectProtocol for WrappedImageHandle {
    fn __str__(&self) -> PyResult<String> {
        debug_str(&self.0)
    }
}
