use std::borrow::Cow;
use std::os::raw::c_int;
use std::ptr::null_mut;

use iced::Font;
use parking_lot::{const_mutex, Mutex};
use pyo3::exceptions::{PyNotImplementedError, PyValueError};
use pyo3::ffi::{PyBUF_FORMAT, PyBUF_ND, PyBUF_STRIDES, PyBUF_WRITABLE, Py_buffer};
use pyo3::prelude::*;
use pyo3::{AsPyPointer, PyBufferProtocol};

use crate::common::debug_str;
use crate::format_to_cow;

struct NameAndData {
    name: String,
    bytes: Vec<u8>,
}

static KNOWN_FONTS: Mutex<Vec<NameAndData>> = const_mutex(Vec::new());

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedFont>()?;
    Ok(())
}

/// Font(name, data)
/// --
///
/// A font.
///
/// The font does not get loaded multiple times, but instead the name is used to tell fonts apart.
/// So you should use the same name for the same data in subsequent Font instance creations.
///
/// Parameters
/// ----------
/// name : str
///     The name of the external font
/// data : bytes-like
///     The bytes of the external font
///
/// See also
/// --------
/// `iced::Font <https://docs.rs/iced/0.3.0/iced/enum.Font.html>`_
///
/// Warning
/// -------
/// The font data gets interned!
/// Even of the module is unloaded / reloaded, some memory is lost until the interpreter is restarted.
#[pyclass(name = "Font", module = "pyiced")]
#[derive(Debug, Clone)]
pub(crate) struct WrappedFont(pub Font);

fn font_from_list(font: &NameAndData) -> WrappedFont {
    let name = font.name.as_str();
    let bytes = font.bytes.as_slice();

    // Safety: once inserted, the value is never released.
    let name: &'static _ = unsafe { std::mem::transmute(name) };
    let bytes: &'static _ = unsafe { std::mem::transmute(bytes) };

    let font = Font::External { name, bytes };
    WrappedFont(font)
}

#[pymethods]
impl WrappedFont {
    #[new]
    fn new(name: &str, data: &[u8]) -> Self {
        let mut guard = KNOWN_FONTS.lock();
        let list = &mut *guard;

        for font in list.iter_mut() {
            if font.name == name {
                return font_from_list(font);
            }
        }

        list.push(NameAndData {
            name: name.to_owned(),
            bytes: data.to_owned(),
        });
        font_from_list(list.last().unwrap())
    }

    /// The default font. This is normally a font configured in a renderer or loaded from the system.
    #[classattr]
    #[allow(non_snake_case)]
    fn DEFAULT() -> Self {
        Self(Font::Default)
    }

    /// Name of the font
    ///
    /// Returns
    /// -------
    /// str
    ///     The name of the font.
    /// None
    ///     For :py:attr:`~pyiced.Font.DEFAULT`.
    #[getter]
    fn name(&self) -> Option<&'static str> {
        match self.0 {
            Font::External { name, .. } => Some(name),
            Font::Default => None,
        }
    }

    /// Bytes data of the font
    ///
    /// Returns
    /// -------
    /// memoryview
    ///     The bytes data of the font.
    /// None
    ///     For :py:attr:`~pyiced.Font.DEFAULT`.
    #[getter]
    fn data(slf: PyRef<Self>, py: Python) -> PyResult<Option<Py<PyAny>>> {
        match slf.0 {
            Font::Default => Ok(None),
            Font::External { .. } => {
                let result = py
                    .import("builtins")?
                    .getattr("memoryview")?
                    .call1((slf,))?
                    .into_py(py);
                Ok(Some(result))
            },
        }
    }

    #[classattr]
    #[allow(non_snake_case)]
    fn __match_args__() -> (&'static str, &'static str) {
        ("name", "data")
    }

    fn __str__(&self) -> PyResult<String> {
        debug_str(&self.0)
    }

    fn __repr__(&self) -> PyResult<Cow<'static, str>> {
        match self.0 {
            Font::Default => Ok(Cow::Borrowed("Font.DEFAULT")),
            Font::External { name, bytes } => {
                format_to_cow!("Font({:?}, <{} bytes>)", name, bytes.len())
            },
        }
    }
}

#[pyproto]
impl PyBufferProtocol for WrappedFont {
    fn bf_getbuffer(slf: PyRefMut<Self>, view: *mut Py_buffer, flags: c_int) -> PyResult<()> {
        if (flags & PyBUF_WRITABLE) == PyBUF_WRITABLE {
            return Err(PyValueError::new_err("Font is not writable."));
        }

        let bytes = match (*slf).0 {
            Font::Default => {
                return Err(PyNotImplementedError::new_err(
                    "Buffer not implemented for default font.",
                ));
            },
            Font::External { bytes, .. } => bytes,
        };

        let view = unsafe { &mut *view };
        view.obj = slf.as_ptr();
        view.buf = bytes.as_ptr() as _;
        view.len = bytes.len() as _;
        view.readonly = 1;
        view.ndim = 1;
        view.format = null_mut();
        view.shape = null_mut();
        view.strides = null_mut();
        view.suboffsets = null_mut();
        view.itemsize = 1;

        if (flags & PyBUF_FORMAT) == PyBUF_FORMAT {
            view.format = &b"B\0"[..] as *const _ as _;
        }
        if (flags & PyBUF_ND) == PyBUF_ND {
            view.shape = &mut view.len as _;
        }
        if (flags & PyBUF_STRIDES) == PyBUF_STRIDES {
            view.strides = &mut view.itemsize as _;
        }

        Ok(())
    }

    fn bf_releasebuffer(_slf: PyRefMut<Self>, view: *mut Py_buffer) -> PyResult<()> {
        let view = unsafe { &mut *view };
        view.obj = null_mut();
        view.buf = null_mut();
        view.len = 0;
        Ok(())
    }
}
