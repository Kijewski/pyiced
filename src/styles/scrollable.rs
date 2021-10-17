#![allow(clippy::needless_option_as_deref)]

use iced::scrollable::{Scrollbar, Scroller, StyleSheet};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyString};

use crate::wrapped::WrappedColor;
use crate::{dyn_style_proto, dyn_style_proto_get, extract_multiple, getters};

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedScrollerStyle>()?;
    m.add_class::<WrappedScrollbarStyle>()?;
    m.add_class::<WrappedScrollableStyleSheet>()?;
    Ok(())
}

/// ScrollerStyle(proto=None, **kwargs)
/// --
///
/// The appearance of the scroller of a :func:`~pyiced.scrollable()`.
///
/// Parameters
/// ----------
/// proto : Optional[Union[ScrollerStyle, str]]
///     Source style sheet to clone and modify.
///     Defaults to `iced_style's <https://docs.rs/iced_style/0.3.0/iced_style/>`_ default style.
///
///     The valid string values are "active", "hovered" and "dragging",
///     same as the argument for :class:`~pyiced.ScrollableStyleSheet`.
///
///     None is the same as "active".
/// color : Color
///     The color of the scroller.
/// border_radius : float
///     The border radius of the scroller.
/// border_width : float
///     The border width of the scroller.
/// border_color : Color
///     The border color of the scroller.
///
/// See also
/// --------
/// `iced_style::scrollable::Scroller <https://docs.rs/iced_style/0.3.0/iced_style/scrollable/struct.Scroller.html>`_
#[pyclass(name = "ScrollerStyle", module = "pyiced")]
#[derive(Debug, Clone, Copy)]
pub(crate) struct WrappedScrollerStyle(pub ScrollerStyle);

#[derive(Debug, Clone, Copy)]
pub(crate) struct ScrollerStyle(pub Scroller);

getters! {
    WrappedScrollerStyle => |&WrappedScrollerStyle(ScrollerStyle(ref o))| o,
    color -> "Color" WrappedColor,
    border_radius -> "float" f32,
    border_width -> "float" f32,
    border_color -> "Color" WrappedColor,
}

#[pymethods]
impl WrappedScrollerStyle {
    #[args(prototype = "None", kwargs = "**")]
    #[new]
    fn new(proto: Option<&PyAny>, kwargs: Option<&PyDict>) -> PyResult<Self> {
        let proto =
            dyn_style_proto_get!(proto, |x: Scrollbar| x.scroller, active, hovered, dragging);
        extract_multiple!(
            kwargs,
            ScrollerStyle(proto),
            color,
            border_radius,
            border_width,
            border_color
        )
    }
}

/// ScrollbarStyle(proto=None, **kwargs)
/// --
///
/// The appearance a specific state of a :func:`~pyiced.scrollable()`.
///
/// Parameters
/// ----------
/// proto : Optional[Union[ScrollbarStyle, str]]
///     Source style sheet to clone and modify.
///     Defaults to `iced_style's <https://docs.rs/iced_style/0.3.0/iced_style/>`_ default style.
///
///     The valid string values are "active", "hovered" and "dragging",
///     same as the argument for :class:`~pyiced.ScrollableStyleSheet`.
///
///     None is the same as "active".
/// background : Optional[Color]
///     The scrollbar's background color.
/// border_radius : float
///     The scrollbar's border radius.
/// border_width : float
///     The scrollbar's border width.
/// border_color : Color
///     The scrollbar's border color.
/// scroller : ScrollerStyle
///     The scroller of the scrollbar.
///
/// See also
/// --------
/// `iced_style::scrollable::Scrollbar <https://docs.rs/iced_style/0.3.0/iced_style/scrollable/struct.Scrollbar.html>`_
#[pyclass(name = "ScrollbarStyle", module = "pyiced")]
#[derive(Debug, Clone, Copy)]
pub(crate) struct WrappedScrollbarStyle(pub ScrollbarStyle);

#[derive(Debug, Clone, Copy)]
pub(crate) struct ScrollbarStyle(pub Scrollbar);

#[pymethods]
impl WrappedScrollbarStyle {
    #[args(prototype = "None", kwargs = "**")]
    #[new]
    fn new(proto: Option<&PyAny>, kwargs: Option<&PyDict>) -> PyResult<Self> {
        let proto = dyn_style_proto!(proto, active, hovered, dragging);
        extract_multiple!(
            kwargs,
            ScrollbarStyle(proto),
            background,
            border_radius,
            border_width,
            border_color,
            scroller,
        )
    }
}

getters! {
    WrappedScrollbarStyle => |&WrappedScrollbarStyle(ScrollbarStyle(ref o))| o,
    background -> "Optional[Color]" Option<WrappedColor>,
    border_radius -> "float" f32,
    border_width -> "float" f32,
    border_color -> "Color" WrappedColor,
    scroller -> "ScrollerStyle" WrappedScrollerStyle,
}

/// ScrollableStyleSheet(active, hovered=None, dragging=None)
/// --
///
/// The appearance of a :func:`~pyiced.scrollable()`.
///
/// Parameters
/// ----------
/// active : ScrollableStyle
///     Normal style of the scrollable.
/// hovered : Optional[ScrollableStyle]
///     Style of the scrollable when the cursor is hovering over it. Defaults to "active".
/// dragging : Optional[ScrollableStyle]
///     Style of a scrollbar that is being dragged. Defaults to "hovered".
///
/// See also
/// --------
/// `iced::widget::scrollable::StyleSheet <https://docs.rs/iced/0.3.0/iced/widget/scrollable/trait.StyleSheet.html>`_
#[pyclass(name = "ScrollableStyleSheet", module = "pyiced")]
#[derive(Debug, Clone, Copy)]
pub(crate) struct WrappedScrollableStyleSheet(pub ScrollableStyleSheet);

#[derive(Debug, Clone, Copy)]
pub(crate) struct ScrollableStyleSheet {
    active: Scrollbar,
    hovered: Scrollbar,
    dragging: Scrollbar,
}

getters! {
    WrappedScrollableStyleSheet => |&WrappedScrollableStyleSheet(ref o)| o,
    active -> "ScrollbarStyle" WrappedScrollbarStyle,
    hovered -> "ScrollbarStyle" WrappedScrollbarStyle,
    dragging -> "ScrollbarStyle" WrappedScrollbarStyle,
}

#[pymethods]
impl WrappedScrollableStyleSheet {
    #[new]
    fn new(
        active: &WrappedScrollbarStyle,
        hovered: Option<&WrappedScrollbarStyle>,
        dragging: Option<&WrappedScrollbarStyle>,
    ) -> Self {
        let active = active.0.0;
        let hovered = hovered.map_or(active, |s| s.0.0);
        let dragging = dragging.map_or(hovered, |s| s.0.0);
        Self(ScrollableStyleSheet {
            active,
            hovered,
            dragging,
        })
    }
}

impl StyleSheet for ScrollableStyleSheet {
    fn active(&self) -> Scrollbar {
        self.active
    }

    fn hovered(&self) -> Scrollbar {
        self.hovered
    }

    fn dragging(&self) -> Scrollbar {
        self.dragging
    }
}
