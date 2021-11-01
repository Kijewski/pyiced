use iced::slider::Handle;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::PyObjectProtocol;

use crate::common::debug_str;
use crate::wrapped::{WrappedColor, WrappedSliderHandleShape};

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedSliderHandle>()?;
    Ok(())
}

/// SliderHandle(shape, color, border_width, border_color)
/// --
///
/// The appearance of the handle of a slider.

///
/// Parameters
/// ----------
/// shape : SliderHandleShape
///     The color of the slider_handle.
/// color : Color
///     The width of the slider_handle.
/// border_width : f32
///     The width of the slider_handle.
/// border_color : Color
///     The width of the slider_handle.
///
/// See also
/// --------
/// `iced::widget::slider::Handle <https://docs.rs/iced/0.3.0/iced/widget/slider/struct.Handle.html>`_
#[pyclass(name = "SliderHandle", module = "pyiced")]
#[derive(Debug, Clone)]
pub(crate) struct WrappedSliderHandle(pub Handle);

#[pymethods]
impl WrappedSliderHandle {
    #[new]
    fn new(
        shape: &WrappedSliderHandleShape,
        color: &WrappedColor,
        border_width: f32,
        border_color: &WrappedColor,
    ) -> PyResult<Self> {
        if !border_width.is_finite() || border_width < 0.0 {
            return Err(PyErr::new::<PyValueError, _>(
                "The border_width must be finite and >= 0",
            ));
        }
        Ok(Self(Handle {
            shape: shape.0,
            color: color.0,
            border_width,
            border_color: border_color.0,
        }))
    }
}

#[pyproto]
impl PyObjectProtocol for WrappedSliderHandle {
    fn __str__(&self) -> PyResult<String> {
        debug_str(&self.0)
    }
}
