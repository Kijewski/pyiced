use iced::slider::HandleShape;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::PyObjectProtocol;

use crate::common::debug_str;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedSliderHandleShape>()?;
    Ok(())
}

/// The shape of the handle of a slider.
///
/// See also
/// --------
/// * `iced::widget::slider::SliderHandleShape <https://docs.rs/iced/0.3.0/iced/widget/slider/enum.SliderHandleShape.html>`_
#[pyclass(name = "SliderHandleShape", module = "pyiced")]
#[derive(Debug, Clone)]
pub(crate) struct WrappedSliderHandleShape(pub HandleShape);

#[pymethods]
impl WrappedSliderHandleShape {
    /// circle($module, /, radius)
    /// --
    ///
    /// A circle.
    ///
    /// Parameters
    /// ----------
    /// radius : f32
    ///     The radius of the circle
    ///
    /// Returns
    /// -------
    /// SliderHandleShape
    ///     A slider handle in the shape of a circle.
    #[staticmethod]
    fn circle(radius: f32) -> PyResult<Self> {
        if !radius.is_finite() || radius < 0.0 {
            return Err(PyErr::new::<PyValueError, _>(
                "The width must be finite and >= 0",
            ));
        }
        Ok(Self(HandleShape::Circle { radius }))
    }

    /// rectangle($module, /, width, border_radius)
    /// --
    ///
    /// A circle.
    ///
    /// Parameters
    /// ----------
    /// width : f32
    ///     TODO
    /// border_radius : f32
    ///     TODO
    ///
    /// Returns
    /// -------
    /// SliderHandleShape
    ///     A slider handle in the shape of a circle.
    #[staticmethod]
    fn rectangle(width: u16, border_radius: f32) -> PyResult<Self> {
        if !border_radius.is_finite() || border_radius < 0.0 {
            return Err(PyErr::new::<PyValueError, _>(
                "The border_radius must be finite and >= 0",
            ));
        }
        Ok(Self(HandleShape::Rectangle {
            width,
            border_radius,
        }))
    }
}

#[pyproto]
impl PyObjectProtocol for WrappedSliderHandleShape {
    fn __str__(&self) -> PyResult<String> {
        debug_str(&self.0)
    }
}
