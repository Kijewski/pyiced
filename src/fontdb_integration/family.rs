use fontdb::Family;
use pyo3::prelude::*;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedFontFamily>()?;
    Ok(())
}

/// A font family.
///
/// See also
/// --------
/// `fontdb::Family <https://docs.rs/fontdb/0.7.0/fontdb/enum.Family.html>`_
#[pyclass(name = "FontFamily", module = "pyiced", freelist = 5)]
#[derive(Debug, Clone)]
pub(crate) struct WrappedFontFamily(pub FamilyEnum);

#[derive(Debug, Clone, Copy)]
pub(crate) enum FamilyEnum {
    Serif,
    SansSerif,
    Cursive,
    Fantasy,
    Monospace,
}

impl From<FamilyEnum> for Family<'static> {
    fn from(value: FamilyEnum) -> Self {
        match value {
            FamilyEnum::Serif => Family::Serif,
            FamilyEnum::SansSerif => Family::SansSerif,
            FamilyEnum::Cursive => Family::Cursive,
            FamilyEnum::Fantasy => Family::Fantasy,
            FamilyEnum::Monospace => Family::Monospace,
        }
    }
}

#[pymethods]
impl WrappedFontFamily {
    /// Serif fonts represent the formal text style for a script.
    #[classattr]
    #[allow(non_snake_case)]
    fn SERIF() -> Self {
        Self(FamilyEnum::Serif)
    }

    /// Glyphs in sans-serif fonts, as the term is used in CSS, are generally low contrast and have stroke endings that are plain — without any flaring, cross stroke, or other ornamentation.
    #[classattr]
    #[allow(non_snake_case)]
    fn SANSSERIF() -> Self {
        Self(FamilyEnum::SansSerif)
    }

    /// Glyphs in cursive fonts generally use a more informal script style, and the result looks more like handwritten pen or brush writing than printed letterwork.
    #[classattr]
    #[allow(non_snake_case)]
    fn CURSIVE() -> Self {
        Self(FamilyEnum::Cursive)
    }

    /// Fantasy fonts are primarily decorative or expressive fonts that contain decorative or expressive representations of characters.
    #[classattr]
    #[allow(non_snake_case)]
    fn FANTASY() -> Self {
        Self(FamilyEnum::Fantasy)
    }

    /// The sole criterion of a monospace font is that all glyphs have the same fixed width.
    #[classattr]
    #[allow(non_snake_case)]
    fn MONOSPACE() -> Self {
        Self(FamilyEnum::Monospace)
    }

    fn __repr__(&self) -> &'static str {
        match self.0 {
            FamilyEnum::Serif => "FontFamily.SERIF",
            FamilyEnum::SansSerif => "FontFamily.SANSSERIF",
            FamilyEnum::Cursive => "FontFamily.CURSIVE",
            FamilyEnum::Fantasy => "FontFamily.FANTASY",
            FamilyEnum::Monospace => "FontFamily.MONOSPACE",
        }
    }
}
