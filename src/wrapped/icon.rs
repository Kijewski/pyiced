use std::path::PathBuf;

use iced::window::Icon;
use image::io::Reader;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use crate::common::debug_str;
use crate::format_to_string_ignore;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedIcon>()?;
    Ok(())
}

/// Icon(path)
/// --
///
/// The icon of a window.
///
/// Parameters
/// ----------
/// path : Path
///     Path to load the icon from. Should be a PNG or BMP file.
///
/// See also
/// --------
/// `iced::window::icon::Icon <https://docs.rs/iced/0.3.0/iced/window/icon/struct.Icon.html>`_
#[pyclass(name = "Icon", module = "pyiced")]
#[derive(Debug, Clone)]
pub(crate) struct WrappedIcon(pub Icon);

#[pymethods]
impl WrappedIcon {
    #[new]
    fn new(path: PathBuf) -> PyResult<Self> {
        let reader = match Reader::open(path) {
            Ok(reader) => reader,
            Err(err) => {
                let msg = format_to_string_ignore!("Could not open icon: {}", err);
                return Err(PyErr::new::<PyValueError, _>(msg));
            },
        };
        let img = match reader.decode() {
            Ok(image) => image.to_rgba8(),
            Err(err) => {
                let msg = format_to_string_ignore!("Could not decode icon: {}", err);
                return Err(PyErr::new::<PyValueError, _>(msg));
            },
        };
        let icon = match Icon::from_rgba(img.to_vec(), img.width(), img.height()) {
            Ok(icon) => icon,
            Err(err) => {
                let msg = format_to_string_ignore!("Could not convert icon: {}", err);
                return Err(PyErr::new::<PyValueError, _>(msg));
            },
        };
        Ok(Self(icon))
    }

    fn __str__(&self) -> PyResult<String> {
        debug_str(&self.0)
    }
}
