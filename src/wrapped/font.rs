use std::borrow::Cow;

use iced::Font;
use parking_lot::{const_mutex, Mutex};
use pyo3::prelude::*;

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

    fn __str__(&self) -> PyResult<String> {
        debug_str(&self.0)
    }

    fn __repr__(&self) -> PyResult<Cow<'static, str>> {
        match self.0 {
            Font::Default => Ok(Cow::Borrowed("Font.DEFAULT")),
            Font::External { name, .. } => format_to_cow!("Font({:?}, …)", name),
        }
    }
}
