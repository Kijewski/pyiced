use iced::progress_bar::{Style, StyleSheet};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::extract_multiple;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedProgressBarStyle>()?;
    Ok(())
}

/// ProgressBarStyleSheet(proto=None, **kwargs)
/// --
///
/// The appearance of a progress_bar.
///
/// All parameters are named parameters and optional.
///
/// Parameters
/// ----------
/// background : Optional[Color]
///     The progress bar's background color.
/// bar : Optional[Color]
///     The progress bar's foreground color.
/// border_radius : float
///     The progress bar's border radius.
///
/// See also
/// --------
/// * `iced::widget::progress_bar::Style <https://docs.rs/iced/0.3.0/iced/widget/progress_bar/struct.Style.html>`_
/// * `iced::widget::progress_bar::StyleSheet <https://docs.rs/iced/0.3.0/iced/widget/progress_bar/trait.StyleSheet.html>`_
#[pyclass(name = "ProgressBarStyleSheet", module = "pyiced")]
#[derive(Debug, Clone, Default, Copy)]
pub(crate) struct WrappedProgressBarStyle(pub ProgressBarStyle);

#[derive(Debug, Clone, Copy)]
pub(crate) struct ProgressBarStyle(pub Style);

impl Default for ProgressBarStyle {
    fn default() -> Self {
        Self(Box::<dyn StyleSheet>::default().style())
    }
}

#[pymethods]
impl WrappedProgressBarStyle {
    #[args(proto = "None", kwargs = "**")]
    #[new]
    fn new(proto: Option<&Self>, kwargs: Option<&PyDict>) -> PyResult<Self> {
        let proto = proto.map_or_else(ProgressBarStyle::default, |p| p.0);
        extract_multiple!(kwargs, proto, background, bar, border_radius,)
    }
}

impl StyleSheet for ProgressBarStyle {
    fn style(&self) -> Style {
        self.0
    }
}
