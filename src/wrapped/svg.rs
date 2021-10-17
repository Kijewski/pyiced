use std::path::PathBuf;

use iced::svg::Handle;
use pyo3::prelude::*;

use crate::common::debug_str;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedSvgHandle>()?;
    Ok(())
}

/// An :func:`~pyiced.svg` handle.
///
/// See also
/// --------
/// `iced::widget::svg::Handle <https://docs.rs/iced/0.3.0/iced/widget/svg/struct.Handle.html>`_
#[pyclass(name = "SvgHandle", module = "pyiced")]
#[derive(Debug, Clone)]
pub(crate) struct WrappedSvgHandle(pub Handle);

#[pymethods]
impl WrappedSvgHandle {
    /// from_path(path)
    /// --
    ///
    /// Creates an SVG Handle pointing to the vector image of the given path.
    ///
    /// Parameters
    /// ----------
    /// path : path-like
    ///     Creates an SVG Handle pointing to the vector image of the given path.
    ///
    /// Returns
    /// -------
    /// SvgHandle
    ///     An SVG handle usable in :func:`~pyiced.svg`.
    #[staticmethod]
    fn from_path(path: PathBuf) -> Self {
        Self(Handle::from_path(path))
    }

    /// from_memory(bytes)
    /// --
    ///
    /// Creates an SVG Handle pointing to the vector image of the given path.
    ///
    /// Parameters
    /// ----------
    /// bytes : bytes-like
    ///     Creates an SVG Handle from raw bytes containing either an SVG string or gzip compressed data.
    ///
    ///     This is useful if you already have your SVG data in-memory, maybe because you downloaded or generated it procedurally.
    ///
    /// Returns
    /// -------
    /// SvgHandle
    ///     An SVG handle usable in :func:`~pyiced.svg`.
    #[staticmethod]
    fn from_memory(bytes: Vec<u8>) -> Self {
        Self(Handle::from_memory(bytes))
    }

    fn __str__(&self) -> PyResult<String> {
        debug_str(&self.0)
    }
}
