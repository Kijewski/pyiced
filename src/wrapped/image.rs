use std::path::PathBuf;

use iced::image::Handle;
use pyo3::prelude::*;

use crate::common::debug_str;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedImageHandle>()?;
    Ok(())
}

/// An :func:`pyiced.image` handle.
///
/// See also
/// --------
/// `iced_native::widget::image::Handle <https://docs.rs/iced_native/0.4.0/iced_native/widget/image/struct.Handle.html>`_
#[pyclass(name = "ImageHandle", module = "pyiced")]
#[derive(Debug, Clone)]
pub(crate) struct WrappedImageHandle(pub Handle);

#[pymethods]
impl WrappedImageHandle {
    /// from_path(path)
    /// --
    ///
    /// Creates an image handle pointing to the image of the given path.
    ///
    /// Arguments
    /// ---------
    /// path : pathlib.Path
    ///     The path of the image file.
    ///
    /// Returns
    /// -------
    /// ImageHandle
    ///     The new image handle.
    #[staticmethod]
    fn from_path(path: PathBuf) -> Self {
        Self(Handle::from_path(path))
    }

    /// from_memory(bytes)
    /// --
    ///
    /// Creates an image handle containing the image data directly.
    ///
    /// Arguments
    /// ---------
    /// bytes : bytes-like
    ///     The data of the image file.
    ///
    /// Returns
    /// -------
    /// ImageHandle
    ///     The new image handle.
    #[staticmethod]
    fn from_memory(bytes: Vec<u8>) -> Self {
        Self(Handle::from_memory(bytes))
    }

    fn __str__(&self) -> PyResult<String> {
        debug_str(&self.0)
    }
}
