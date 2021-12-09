use std::borrow::Cow;
use std::sync::Arc;

use fontdb::{Database, ID};
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

use crate::fontdb_integration::{WrappedFontStretch, WrappedFontStyle, WrappedFontWeight};
use crate::format_to_string_ignore;
use crate::wrapped::{font_from_list, NameAndData, WrappedFont, KNOWN_FONTS};

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedFontId>()?;
    Ok(())
}

fn expired() -> PyErr {
    PyErr::new::<PyRuntimeError, _>("Font info expired")
}

/// A unique per database face ID.
///
/// See also
/// --------
/// `fontdb::ID <https://docs.rs/fontdb/0.7.0/fontdb/struct.ID.html>`_
#[pyclass(name = "FontId", module = "pyiced", freelist = 1)]
#[derive(Debug, Clone)]
pub(crate) struct WrappedFontId {
    pub id: ID,
    pub arc: Arc<Database>,
}

#[pymethods]
impl WrappedFontId {
    /// load($self)
    /// --
    ///
    /// Loads the referenced font into memory.
    ///
    /// Returns
    /// -------
    /// Font
    ///     The Font object to be used in e.g. :meth:`~pyiced.IcedApp.view()`.
    fn load(&self) -> PyResult<WrappedFont> {
        let id = self.id;
        let db = self.arc.as_ref();
        let name = match db.face(id) {
            Some(info) => info.post_script_name.as_str(),
            _ => return Err(expired()),
        };

        let mut fonts_guard = KNOWN_FONTS.lock();
        let list = &mut *fonts_guard;
        for font in list.iter_mut() {
            if font.name == name {
                return Ok(font_from_list(font));
            }
        }

        let font = db
            .with_face_data(id, |data, _| NameAndData {
                name: name.to_owned(),
                bytes: data.to_owned(),
            })
            .ok_or_else(expired)?;
        list.push(font);
        Ok(font_from_list(list.last().unwrap()))
    }

    /// Corresponds to a Font Family in a TrueType font.
    #[getter]
    fn family(&self) -> PyResult<String> {
        self.arc
            .as_ref()
            .face(self.id)
            .ok_or_else(expired)
            .map(|info| info.family.clone())
    }

    /// Corresponds to a PostScript name in a TrueType font.
    #[getter]
    fn name(&self) -> PyResult<String> {
        self.arc
            .as_ref()
            .face(self.id)
            .ok_or_else(expired)
            .map(|info| info.post_script_name.clone())
    }

    /// A font face style.
    #[getter]
    fn style(&self) -> PyResult<WrappedFontStyle> {
        self.arc
            .as_ref()
            .face(self.id)
            .ok_or_else(expired)
            .map(|info| WrappedFontStyle(info.style))
    }

    /// A font face weight.
    #[getter]
    fn weight(&self) -> PyResult<WrappedFontWeight> {
        self.arc
            .as_ref()
            .face(self.id)
            .ok_or_else(expired)
            .map(|info| WrappedFontWeight(info.weight))
    }

    /// A font face stretch.
    #[getter]
    fn stretch(&self) -> PyResult<WrappedFontStretch> {
        self.arc
            .as_ref()
            .face(self.id)
            .ok_or_else(expired)
            .map(|info| WrappedFontStretch(info.stretch))
    }

    /// Indicates that the font face is monospaced.
    #[getter]
    fn monospaced(&self) -> PyResult<bool> {
        self.arc
            .as_ref()
            .face(self.id)
            .ok_or_else(expired)
            .map(|info| info.monospaced)
    }

    fn __repr__(&self) -> Cow<str> {
        match self.arc.as_ref().face(self.id) {
            Some(info) => format_to_string_ignore!(
                "FontId(name={name:?}, family={family:?}, style={style:?}, weight={weight:?}, stretch={stretch:?})",
                name = info.post_script_name,
                family = info.family,
                style = info.style,
                weight = info.weight,
                stretch = info.stretch,
            ),
            None => Cow::Borrowed("FontId(<expired>)"),
        }
    }

    fn __str__(&self) -> Cow<str> {
        self.__repr__()
    }
}
