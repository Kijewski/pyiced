use iced::slider::{Style, StyleSheet};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::extract_multiple;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedSliderStyle>()?;
    Ok(())
}

/// SliderStyle(prototype=None, **kwargs)
/// --
///
/// The appearance of a slider.
///
/// Parameters
/// ----------
/// prototype : Optional[str]
///     None, "hovered" or "dragging"
/// background : Color
///     The slider' background color.
/// checkmark_color : Color
///     The color of the slider.
/// border_radius : float
///     The slider' border radius.
/// border_width : float
///     The slider' border width.
/// border_color : Color
///     The slider' border color.
///
/// See also
/// --------
/// * `iced::widget::slider::Style <https://docs.rs/iced/0.3.0/iced/widget/slider/struct.Style.html>`_
#[pyclass(name = "SliderStyle", module = "pyiced")]
#[derive(Debug, Clone, Copy)]
pub(crate) struct WrappedSliderStyle(pub SliderStyle);

#[derive(Debug, Clone, Copy)]
pub(crate) struct SliderStyle(pub Style);

#[derive(Debug, Clone, Copy)]
pub(crate) struct SliderStyles {
    pub active: SliderStyle,
    pub hovered: SliderStyle,
    pub dragging: SliderStyle,
}

impl Default for SliderStyles {
    fn default() -> Self {
        let proto = Box::<dyn StyleSheet>::default();
        Self {
            active: SliderStyle(proto.active()),
            hovered: SliderStyle(proto.hovered()),
            dragging: SliderStyle(proto.dragging()),
        }
    }
}

#[pymethods]
impl WrappedSliderStyle {
    #[args(prototype = "None", kwargs = "**")]
    #[new]
    fn new(prototype: Option<&str>, kwargs: Option<&PyDict>) -> PyResult<Self> {
        let proto = Box::<dyn StyleSheet>::default();
        let proto = match prototype {
            None => proto.active(),
            Some("hovered") => proto.hovered(),
            Some("dragging") => proto.dragging(),
            _ => {
                return Err(PyErr::new::<PyValueError, _>(
                    "Valid prototypes are 'normal', 'hovered' and 'dragging'.",
                ));
            },
        };
        extract_multiple!(kwargs, SliderStyle(proto), rail_colors, handle,)
    }
}

impl StyleSheet for SliderStyles {
    fn active(&self) -> Style {
        self.active.0
    }

    fn hovered(&self) -> Style {
        self.hovered.0
    }

    fn dragging(&self) -> Style {
        self.dragging.0
    }
}

impl SliderStyles {
    pub(crate) fn new(
        style: Option<&WrappedSliderStyle>,
        style_hoverer: Option<&WrappedSliderStyle>,
        style_dragging: Option<&WrappedSliderStyle>,
    ) -> Option<Self> {
        match (style, style_hoverer, style_dragging) {
            (None, None, None) => None,
            (Some(active), None, None) => Some(SliderStyles {
                active: active.0,
                ..Default::default()
            }),
            (None, Some(hovered), None) => Some(SliderStyles {
                hovered: hovered.0,
                ..Default::default()
            }),
            (Some(active), Some(hovered), None) => Some(SliderStyles {
                active: active.0,
                hovered: hovered.0,
                ..Default::default()
            }),
            (None, None, Some(dragging)) => Some(SliderStyles {
                dragging: dragging.0,
                ..Default::default()
            }),
            (Some(active), None, Some(dragging)) => Some(SliderStyles {
                active: active.0,
                dragging: dragging.0,
                ..Default::default()
            }),
            (None, Some(hovered), Some(dragging)) => Some(SliderStyles {
                hovered: hovered.0,
                dragging: dragging.0,
                ..Default::default()
            }),
            (Some(active), Some(hovered), Some(dragging)) => Some(SliderStyles {
                active: active.0,
                hovered: hovered.0,
                dragging: dragging.0,
            }),
        }
    }
}
