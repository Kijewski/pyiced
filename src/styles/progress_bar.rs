use iced::progress_bar::{Style, StyleSheet};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::extract_multiple;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedProgressBarStyle>()?;
    Ok(())
}

/// The appearance of a progress_bar.
///
/// All parameters are named parameters and optional.
///
/// Parameters
/// ----------
/// background : Option[Color]
///     The progress bar's background color.
/// bar : Option[Color]
///     The progress bar's foreground color.
/// border_radius : float
///     The progress bar's border radius.
///
/// See also
/// --------
/// * `iced::widget::progress_bar::Style <https://docs.rs/iced/0.3.0/iced/widget/progress_bar/struct.Style.html>`_
#[pyclass(name = "ProgressBarStyle", module = "pyiced")]
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
    #[args(kwargs = "**")]
    #[new]
    fn new(kwargs: Option<&PyDict>) -> PyResult<Self> {
        extract_multiple!(
            kwargs,
            ProgressBarStyle::default(),
            background,
            bar,
            border_radius,
        )
    }
}

impl StyleSheet for ProgressBarStyle {
    fn style(&self) -> Style {
        self.0
    }
}
