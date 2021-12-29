#![allow(clippy::needless_option_as_deref)]

use iced::slider::{Handle, Style, StyleSheet};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyString};

use crate::common::debug_str;
use crate::wrapped::color::{ColorFormat, WrappedColor};
use crate::wrapped::slider_handle_shape::{SliderHandleShapeFormat, WrappedSliderHandleShape};
use crate::{extract_multiple, format_to_py, getters};

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedSliderHandle>()?;
    Ok(())
}

/// SliderHandle(proto, **kwargs)
/// --
///
/// The appearance of the handle of a slider.
///
/// Parameters
/// ----------
/// proto : Optional[Union[SliderHandle, str]]
///     Source style sheet to clone and modify.
///     Defaults to `iced_style's <https://docs.rs/iced_style/0.3.0/iced_style/>`_ default style.
///
///     The valid string values are "active", "hovered" and "dragging",
///     same as the argument for :class:`~pyiced.SliderStyleSheet`.
///
///     None is the same as "active".
/// shape : SliderHandleShape
///     The color of the slider_handle.
/// color : Color
///     The width of the slider_handle.
/// border_width : float
///     The width of the slider_handle.
/// border_color : Color
///     The width of the slider_handle.
///
/// See also
/// --------
/// `iced::widget::slider::Handle <https://docs.rs/iced/0.3.0/iced/widget/slider/struct.Handle.html>`_
#[pyclass(name = "SliderHandle", module = "pyiced")]
#[derive(Debug, Clone)]
pub(crate) struct WrappedSliderHandle(pub SliderHandle);

#[derive(Debug, Clone)]
pub(crate) struct SliderHandle(pub Handle);

getters! {
    WrappedSliderHandle => |&WrappedSliderHandle(SliderHandle(ref o))| o,
    color -> "Color" WrappedColor,
    border_width -> "float" f32,
    border_color -> "Color" WrappedColor,
}

#[pymethods]
impl WrappedSliderHandle {
    #[new]
    #[args(prototype = "None", kwargs = "**")]
    fn new(proto: Option<&PyAny>, kwargs: Option<&PyDict>) -> PyResult<Self> {
        let proto =
            crate::dyn_style_proto_get!(proto, |x: Style| x.handle, active, hovered, dragging);
        extract_multiple!(
            kwargs,
            SliderHandle(proto),
            shape,
            color,
            border_width,
            border_color,
        )
    }

    fn __str__(&self) -> PyResult<String> {
        debug_str(&self.0)
    }

    fn __repr__(&self) -> PyResult<String> {
        let Handle {
            ref shape,
            ref color,
            border_width,
            ref border_color,
        } = self.0.0;
        format_to_py!(
            "SliderHandle({}, {}, {:?}, {})",
            SliderHandleShapeFormat(shape),
            ColorFormat(color),
            border_width,
            ColorFormat(border_color),
        )
    }

    fn shape(&self) -> WrappedSliderHandleShape {
        WrappedSliderHandleShape(self.0.0.shape)
    }
}
