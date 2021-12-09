use std::borrow::Cow;

use fontdb::Weight;
use pyo3::prelude::*;

use crate::format_to_string_ignore;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedFontWeight>()?;
    Ok(())
}

/// Specifies the weight of glyphs in the font, their degree of blackness or stroke thickness.
///
/// See also
/// --------
/// `fontdb::Weight <https://docs.rs/fontdb/0.7.0/fontdb/struct.Weight.html>`_
#[pyclass(name = "FontWeight", module = "pyiced", freelist = 9)]
#[derive(Debug, Clone)]
pub(crate) struct WrappedFontWeight(pub Weight);

#[pymethods]
impl WrappedFontWeight {
    #[new]
    fn new(value: u16) -> Self {
        Self(Weight(value))
    }

    /// Thin weight (100), the thinnest value.
    #[classattr]
    #[allow(non_snake_case)]
    fn THIN() -> Self {
        Self(Weight::THIN)
    }

    /// Extra light weight (200).
    #[classattr]
    #[allow(non_snake_case)]
    fn EXTRALIGHT() -> Self {
        Self(Weight::EXTRA_LIGHT)
    }

    /// Light weight (300).
    #[classattr]
    #[allow(non_snake_case)]
    fn LIGHT() -> Self {
        Self(Weight::LIGHT)
    }

    /// Normal (400).
    #[classattr]
    #[allow(non_snake_case)]
    fn NORMAL() -> Self {
        Self(Weight::NORMAL)
    }

    /// Medium weight (500, higher than normal).
    #[classattr]
    #[allow(non_snake_case)]
    fn MEDIUM() -> Self {
        Self(Weight::MEDIUM)
    }

    /// Semibold weight (600).
    #[classattr]
    #[allow(non_snake_case)]
    fn SEMIBOLD() -> Self {
        Self(Weight::SEMIBOLD)
    }

    /// Bold weight (700).
    #[classattr]
    #[allow(non_snake_case)]
    fn BOLD() -> Self {
        Self(Weight::BOLD)
    }

    /// Extra-bold weight (800).
    #[classattr]
    #[allow(non_snake_case)]
    fn EXTRABOLD() -> Self {
        Self(Weight::EXTRA_BOLD)
    }

    /// Black weight (900), the thickest value.
    #[classattr]
    #[allow(non_snake_case)]
    fn BLACK() -> Self {
        Self(Weight::BLACK)
    }

    fn __repr__(&self) -> Cow<str> {
        match self.0 {
            Weight::THIN => Cow::Borrowed("FontWeight.THIN"),
            Weight::EXTRA_LIGHT => Cow::Borrowed("FontWeight.EXTRALIGHT"),
            Weight::LIGHT => Cow::Borrowed("FontWeight.LIGHT"),
            Weight::NORMAL => Cow::Borrowed("FontWeight.NORMAL"),
            Weight::MEDIUM => Cow::Borrowed("FontWeight.MEDIUM"),
            Weight::SEMIBOLD => Cow::Borrowed("FontWeight.SEMIBOLD"),
            Weight::BOLD => Cow::Borrowed("FontWeight.BOLD"),
            Weight::EXTRA_BOLD => Cow::Borrowed("FontWeight.EXTRABOLD"),
            Weight::BLACK => Cow::Borrowed("FontWeight.BLACK"),
            Weight(n) => format_to_string_ignore!("FontWeight({:?})", n),
        }
    }

    fn __str__(&self) -> Cow<str> {
        self.__repr__()
    }

    #[getter]
    fn value(&self) -> u16 {
        self.0.0
    }

    fn __index__(&self) -> u16 {
        self.0.0
    }

    fn __int__(&self) -> u16 {
        self.0.0
    }

    #[classattr]
    #[allow(non_snake_case)]
    fn __match_args__() -> (&'static str,) {
        ("value",)
    }
}
