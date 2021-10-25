use iced::pane_grid::{Line, StyleSheet};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::extract_multiple;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedPaneGridStyle>()?;
    Ok(())
}

/// The appearance of a pane_grid.
///
/// All parameters are named parameters and optional.
///
/// Parameters
/// ----------
/// picked_split : Optional[Line]
///     The line to draw when a split is picked.
/// hovered_split : Optional[Line]
///     The line to draw when a split is hovered.
///
/// See also
/// --------
/// * `iced::widget::pane_grid::Style <https://docs.rs/iced/0.3.0/iced/widget/pane_grid/trait.StyleSheet.html>`_
#[pyclass(name = "PaneGridStyle", module = "pyiced")]
#[derive(Debug, Clone, Default)]
pub(crate) struct WrappedPaneGridStyle(pub PaneGridStyle);

#[derive(Debug, Clone, Default)]
pub(crate) struct PaneGridStyle(pub PaneGridStyleInner);

#[derive(Debug, Clone, Default)]
pub(crate) struct PaneGridStyleInner {
    picked_split: Option<Line>,
    hovered_split: Option<Line>,
}

#[pymethods]
impl WrappedPaneGridStyle {
    #[args(kwargs = "**")]
    #[new]
    fn new(kwargs: Option<&PyDict>) -> PyResult<Self> {
        extract_multiple!(
            kwargs,
            PaneGridStyle::default(),
            picked_split,
            hovered_split,
        )
    }
}

impl StyleSheet for WrappedPaneGridStyle {
    fn picked_split(&self) -> Option<Line> {
        self.0.0.picked_split
    }

    fn hovered_split(&self) -> Option<iced::pane_grid::Line> {
        self.0.0.hovered_split
    }
}
