use iced::slider::Handle;
use pyo3::prelude::*;

use crate::common::{debug_str, validate_f32_nonneg};
use crate::format_to_py;
use crate::wrapped::color::ColorFormat;
use crate::wrapped::slider_handle_shape::SliderHandleShapeFormat;
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
        Ok(Self(Handle {
            shape: shape.0,
            color: color.0,
            border_width: validate_f32_nonneg(border_width)?,
            border_color: border_color.0,
        }))
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
        } = self.0;
        format_to_py!(
            "SliderHandle({}, {}, {:?}, {})",
            SliderHandleShapeFormat(shape),
            ColorFormat(color),
            border_width,
            ColorFormat(border_color),
        )
    }
}
