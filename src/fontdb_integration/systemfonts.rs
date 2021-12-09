use std::sync::Arc;

use fontdb::{Database, Family, Query, Stretch, Style, Weight};
use parking_lot::{const_mutex, Mutex};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyString;
use pyo3::wrap_pyfunction;

use crate::common::EitherPy::{self, Left, Right};
use crate::fontdb_integration::family::FamilyEnum;
use crate::fontdb_integration::{
    WrappedFontFamily, WrappedFontId, WrappedFontStretch, WrappedFontStyle, WrappedFontWeight,
};
use crate::format_to_string_ignore;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(findfont, m)?)?;
    m.add_function(wrap_pyfunction!(systemfonts, m)?)?;
    Ok(())
}

static DATABASE: Mutex<Option<Arc<Database>>> = const_mutex(None);

fn get_arc() -> Arc<Database> {
    let mut guard = DATABASE.lock();
    let opt = &mut *guard;
    if let Some(arc) = opt {
        return arc.clone();
    }

    let mut db = Database::new();
    db.load_system_fonts();
    let arc = Arc::new(db);
    *opt = Some(arc.clone());
    arc
}

fn py_to_fontfamily(
    family: EitherPy<WrappedFontFamily, &PyString>,
) -> PyResult<EitherPy<FamilyEnum, String>> {
    match family {
        Left(WrappedFontFamily(family)) => Ok(Left(family)),
        Right(family) => match family.to_str()? {
            "serif" => Ok(Left(FamilyEnum::Serif)),
            "sans-serif" => Ok(Left(FamilyEnum::SansSerif)),
            "cursive" => Ok(Left(FamilyEnum::Cursive)),
            "fantasy" => Ok(Left(FamilyEnum::Fantasy)),
            "monospace" => Ok(Left(FamilyEnum::Monospace)),
            s => Ok(Right(s.to_owned())),
        },
    }
}

/// findfont($module, /, family=None, weight=None, stretch=None, style=None)
/// --
///
/// Performs a CSS-like query and returns the best matched font face.
///
/// Arguments can be given using their constants or using their CSS value, e.g.
///
/// .. code:: python
///
///     >>> from pyiced import  *
///     >>> findfont("serif", "extra-light", "normal", "italic")
///     FontId(name="TimesNewRomanPS-ItalicMT", family="Times New Roman",
///            style=Italic, weight=Weight(400), stretch=Normal)
///
/// Arguments
/// ---------
/// families : Union[FontFamily, str, Iterable[Union[FontFamily, str]], None]
///     A prioritized (list of) font family names or generic family name(s).
///     Defaults to :attr:`~pyiced.FontFamily.SANSSERIF`.
/// weight : Union[FontWeight, int, str, None]
///     Specifies the weight of glyphs in the font, their degree of blackness or stroke thickness.
///     Defaults to :attr:`~pyiced.FontWeight.NORMAL`.
/// stretch : Union[FontStretch, str, None]
///     Selects a normal, condensed, or expanded face from a font family.
///     Defaults to :attr:`~pyiced.FontStretch.NORMAL`.
/// style : Union[FontStyle, str, None]
///     Allows italic or oblique faces to be selected.
///     Defaults to :attr:`~pyiced.FontStyle.NORMAL`.
///
/// Returns
/// -------
/// Optional[FontId]
///     The best match found, if one was found.
///
/// See also
/// --------
/// `fontdb::Query <https://docs.rs/fontdb/0.7.0/fontdb/struct.Query.html>`_
#[pyfunction(name = "findfont")]
fn findfont(
    families: Option<EitherPy<EitherPy<WrappedFontFamily, &PyString>, &PyAny>>,
    weight: Option<EitherPy<WrappedFontWeight, EitherPy<u16, &PyString>>>,
    stretch: Option<EitherPy<WrappedFontStretch, &PyString>>,
    style: Option<EitherPy<WrappedFontStyle, &PyString>>,
) -> PyResult<Option<WrappedFontId>> {
    let families: Vec<EitherPy<FamilyEnum, String>> = match families {
        None => vec![],
        Some(Left(family)) => vec![py_to_fontfamily(family)?],
        Some(Right(families)) => families
            .iter()?
            .map(|family| py_to_fontfamily(family?.extract()?))
            .take(64)
            .collect::<PyResult<_>>()?,
    };
    let families: Vec<Family<'_>> = families
        .iter()
        .map(|family| match family {
            Left(family) => Family::from(*family),
            Right(family) => Family::Name(family.as_str()),
        })
        .collect();
    let families = match families.is_empty() {
        true => &[Family::SansSerif],
        false => families.as_slice(),
    };

    let weight = match weight {
        Some(Left(WrappedFontWeight(weight))) => weight,
        Some(Right(Left(weight))) => Weight(weight),
        Some(Right(Right(weight))) => match weight.to_str()? {
            "thin" => Weight::THIN,
            "extra-light" => Weight::EXTRA_LIGHT,
            "light" => Weight::LIGHT,
            "normal" => Weight::NORMAL,
            "medium" => Weight::MEDIUM,
            "semibold" => Weight::SEMIBOLD,
            "bold" => Weight::BOLD,
            "extra-bold" => Weight::EXTRA_BOLD,
            "black" => Weight::BLACK,
            s => {
                return Err(PyErr::new::<PyValueError, _>(format_to_string_ignore!(
                    "Unknown font-weight: {:?}",
                    s,
                )));
            },
        },
        None => Weight::NORMAL,
    };

    let stretch = match stretch {
        Some(Left(WrappedFontStretch(stretch))) => stretch,
        Some(Right(stretch)) => match stretch.to_str()? {
            "ultra-condensed" => Stretch::UltraCondensed,
            "extra-condensed" => Stretch::ExtraCondensed,
            "condensed" => Stretch::Condensed,
            "semi-condensed" => Stretch::SemiCondensed,
            "normal" => Stretch::Normal,
            "semi-expanded" => Stretch::SemiExpanded,
            "expanded" => Stretch::Expanded,
            "extra-expanded" => Stretch::ExtraExpanded,
            "ultra-expanded" => Stretch::UltraExpanded,
            s => {
                return Err(PyErr::new::<PyValueError, _>(format_to_string_ignore!(
                    "Unknown font-stretch: {:?}",
                    s,
                )));
            },
        },
        None => Stretch::Normal,
    };

    let style = match style {
        Some(Left(WrappedFontStyle(style))) => style,
        Some(Right(style)) => match style.to_str()? {
            "normal" => Style::Normal,
            "italic" => Style::Italic,
            "oblique" => Style::Oblique,
            s => {
                return Err(PyErr::new::<PyValueError, _>(format_to_string_ignore!(
                    "Unknown font-style: {:?}",
                    s,
                )));
            },
        },
        None => Style::Normal,
    };

    let query = Query {
        families,
        weight,
        stretch,
        style,
    };

    let arc = get_arc();
    let result = arc
        .as_ref()
        .query(&query)
        .map(|id| WrappedFontId { id, arc });
    Ok(result)
}

/// systemfonts($self)
/// --
///
/// List loaded system fonts.
///
/// Returns
/// -------
/// Iterator[FontId]
///     An iterator over all system fonts.
#[pyfunction(name = "systemfonts")]
fn systemfonts() -> WrappedSystemFontsIds {
    WrappedSystemFontsIds {
        arc: get_arc(),
        index: 0,
    }
}

#[pyclass(name = "SystemFontsIds", module = "pyiced")]
#[derive(Debug, Clone)]
struct WrappedSystemFontsIds {
    pub arc: Arc<Database>,
    pub index: usize,
}

#[pymethods]
impl WrappedSystemFontsIds {
    fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __next__(&mut self) -> Option<WrappedFontId> {
        let result = WrappedFontId {
            id: self.arc.faces().get(self.index)?.id,
            arc: self.arc.clone(),
        };
        self.index += 1;
        Some(result)
    }
}
