use iced::Font;
use parking_lot::Mutex;
use parking_lot::const_mutex;
use pyo3::prelude::*;
use pyo3::PyObjectProtocol;

use crate::common::debug_str;

struct NameAndData {
    name: String,
    bytes: Vec<u8>,
}

static KNOWN_FONTS: Mutex<Vec<NameAndData>> = const_mutex(Vec::new());

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedFont>()?;
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Private;

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
/// * `iced::Font <https://docs.rs/iced/0.3.0/iced/enum.Font.html>`_
/// 
/// Warning
/// -------
/// The font data gets interned!
/// Even of the module is unloaded / reloaded, some memory is lost until the interpreter is restated.
#[pyclass(name = "Font", module = "pyiced")]
#[derive(Debug, Clone)]
pub(crate) struct WrappedFont(pub Font, Private);

fn font_from_list(font: &NameAndData) -> WrappedFont {
    let name = font.name.as_str();
    let bytes = font.bytes.as_slice();
    let font = Font::External {
        // Safety: once inserted, the value is never released.
        name: unsafe { std::mem::transmute(name) },
        bytes: unsafe { std::mem::transmute(bytes) },
    };
    WrappedFont(font, Private)
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
        Self(Font::Default, Private)
    }
}

#[pyproto]
impl PyObjectProtocol for WrappedFont {
    fn __str__(&self) -> PyResult<String> {
        debug_str(&self.0)
    }
}
