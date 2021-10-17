use iced::pane_grid::{Line, StyleSheet};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::wrapped::WrappedLine;
use crate::{extract_multiple, getters};

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedPaneGridStyle>()?;
    Ok(())
}

/// PaneGridStyleSheet(proto=None, **kwargs)
/// --
///
/// The appearance of a pane_grid.
///
/// Parameters
/// ----------
/// proto : Optional[PaneGridStyleSheet]
///     Source style sheet to clone and modify.
///     Defaults to `iced_style's <https://docs.rs/iced_style/0.3.0/iced_style/>`_ default style.
/// picked_split : Optional[Line]
///     The line to draw when a split is picked.
/// hovered_split : Optional[Line]
///     The line to draw when a split is hovered.
///
/// See also
/// --------
/// * `iced::widget::pane_grid::Style <https://docs.rs/iced/0.3.0/iced/widget/pane_grid/trait.StyleSheet.html>`_
/// * `iced::widget::pane_grid::StyleSheet <https://docs.rs/iced/0.3.0/iced/widget/pane_grid/trait.StyleSheet.html>`_
#[pyclass(name = "PaneGridStyleSheet", module = "pyiced")]
#[derive(Debug, Clone, Default, Copy)]
pub(crate) struct WrappedPaneGridStyle(pub PaneGridStyle);

#[derive(Debug, Clone, Default, Copy)]
pub(crate) struct PaneGridStyle(pub PaneGridStyleInner);

#[derive(Debug, Clone, Default, Copy)]
pub(crate) struct PaneGridStyleInner {
    picked_split: Option<Line>,
    hovered_split: Option<Line>,
}

getters! {
    WrappedPaneGridStyle => |&WrappedPaneGridStyle(PaneGridStyle(ref o))| o,
    picked_split -> "Optional[Line]" Option<WrappedLine>,
    hovered_split -> "Optional[Line]" Option<WrappedLine>,
}

#[pymethods]
impl WrappedPaneGridStyle {
    #[args(proto = "None", kwargs = "**")]
    #[new]
    fn new(proto: Option<&Self>, kwargs: Option<&PyDict>) -> PyResult<Self> {
        let proto = proto.map_or_else(PaneGridStyle::default, |p| p.0);
        extract_multiple!(kwargs, proto, picked_split, hovered_split)
    }
}

impl StyleSheet for WrappedPaneGridStyle {
    fn picked_split(&self) -> Option<Line> {
        self.0.0.picked_split
    }

    fn hovered_split(&self) -> Option<Line> {
        self.0.0.hovered_split
    }
}
