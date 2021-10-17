#![allow(clippy::needless_option_as_deref)]

use iced::slider::{Style, StyleSheet};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyString};

use crate::wrapped::{WrappedColor, WrappedSliderHandle};
use crate::{dyn_style_proto, extract_multiple, getters};

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedSliderStyle>()?;
    m.add_class::<WrappedSliderStyleSheet>()?;
    Ok(())
}

/// SliderStyle(proto=None, **kwargs)
/// --
///
/// The appearance of a slider for some state.
///
/// Parameters
/// ----------
/// proto : Optional[Union[SliderStyle, str]]
///     Source style sheet to clone and modify.
///     Defaults to `iced_style's <https://docs.rs/iced_style/0.3.0/iced_style/>`_ default style.
///
///     The valid string values are "active", "hovered" and "dragging",
///     same as the argument for :class:`~pyiced.SliderStyleSheet`.
///
///     None is the same as "active".
/// rail_colors : Tuple[Color, Color]
///     TODO
/// handle : SliderHandle
///     TODO
///
/// See also
/// --------
/// `iced::widget::slider::Style <https://docs.rs/iced/0.3.0/iced/widget/slider/struct.Style.html>`_
#[pyclass(name = "SliderStyle", module = "pyiced")]
#[derive(Debug, Clone, Copy)]
pub(crate) struct WrappedSliderStyle(pub SliderStyle);

#[derive(Debug, Clone, Copy)]
pub(crate) struct SliderStyle(pub Style);

getters! {
    WrappedSliderStyle => |&WrappedSliderStyle(SliderStyle(ref o))| o,
    rail_colors -> "Tuple[Color, Color]" (WrappedColor, WrappedColor),
    handle -> "SliderHandle" WrappedSliderHandle,
}

#[pymethods]
impl WrappedSliderStyle {
    #[args(prototype = "None", kwargs = "**")]
    #[new]
    fn new(proto: Option<&PyAny>, kwargs: Option<&PyDict>) -> PyResult<Self> {
        let proto = dyn_style_proto!(proto, active, hovered, dragging);
        extract_multiple!(kwargs, SliderStyle(proto), rail_colors, handle)
    }
}

/// SliderStyleSheet(active, hovered=None, dragging=None)
/// --
///
/// The appearance of a slider.
///
/// Parameters
/// ----------
/// active : SliderStyle
///     Normal style of the slider.
/// hovered : Optional[SliderStyle]
///     Style of the slider when the cursor is hovering over it. Defaults to "active".
/// dragging : Optional[SliderStyle]
///     Style of the slider is being dragged. Defaults to "hovered".
///
/// See also
/// --------
/// `iced::widget::slider::StyleSheet <https://docs.rs/iced/0.3.0/iced/widget/slider/trait.StyleSheet.html>`_
#[pyclass(name = "SliderStyleSheet", module = "pyiced")]
#[derive(Debug, Clone, Copy)]
pub(crate) struct WrappedSliderStyleSheet(pub SliderStyleSheet);

#[derive(Debug, Clone, Copy)]
pub(crate) struct SliderStyleSheet {
    active: Style,
    hovered: Style,
    dragging: Style,
}

#[pymethods]
impl WrappedSliderStyleSheet {
    #[new]
    fn new(
        active: &WrappedSliderStyle,
        hovered: Option<&WrappedSliderStyle>,
        dragging: Option<&WrappedSliderStyle>,
    ) -> Self {
        let active = active.0.0;
        let hovered = hovered.map_or(active, |s| s.0.0);
        let dragging = dragging.map_or(hovered, |s| s.0.0);
        Self(SliderStyleSheet {
            active,
            hovered,
            dragging,
        })
    }
}

impl StyleSheet for SliderStyleSheet {
    fn active(&self) -> Style {
        self.active
    }

    fn hovered(&self) -> Style {
        self.hovered
    }

    fn dragging(&self) -> Style {
        self.dragging
    }
}
