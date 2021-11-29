use std::fmt::Display;

use iced::slider::HandleShape;
use pyo3::prelude::*;

use crate::common::{debug_str, validate_f32_nonneg};
use crate::format_to_py;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedSliderHandleShape>()?;
    Ok(())
}

/// The shape of the handle of a slider.
///
/// See also
/// --------
/// `iced::widget::slider::HandleShape <https://docs.rs/iced/0.3.0/iced/widget/slider/enum.HandleShape.html>`_
#[pyclass(name = "SliderHandleShape", module = "pyiced")]
#[derive(Debug, Clone)]
pub(crate) struct WrappedSliderHandleShape(pub HandleShape);

#[pymethods]
impl WrappedSliderHandleShape {
    /// circle(radius)
    /// --
    ///
    /// A circle.
    ///
    /// Parameters
    /// ----------
    /// radius : float
    ///     The radius of the circle
    ///
    /// Returns
    /// -------
    /// SliderHandleShape
    ///     A slider handle in the shape of a circle.
    #[staticmethod]
    fn circle(radius: f32) -> PyResult<Self> {
        Ok(Self(HandleShape::Circle {
            radius: validate_f32_nonneg(radius)?,
        }))
    }

    /// rectangle(width, border_radius)
    /// --
    ///
    /// A rectangle.
    ///
    /// Parameters
    /// ----------
    /// width : float
    ///     The length of an edge.
    /// border_radius : float
    ///     The border radius.
    ///
    /// Returns
    /// -------
    /// SliderHandleShape
    ///     A slider handle in the shape of a rectangle.
    #[staticmethod]
    fn rectangle(width: u16, border_radius: f32) -> PyResult<Self> {
        Ok(Self(HandleShape::Rectangle {
            width,
            border_radius: validate_f32_nonneg(border_radius)?,
        }))
    }

    fn __str__(&self) -> PyResult<String> {
        debug_str(&self.0)
    }

    fn __repr__(&self) -> PyResult<String> {
        format_to_py!("{}", SliderHandleShapeFormat(&self.0))
    }
}

#[derive(Clone)]
pub(crate) struct SliderHandleShapeFormat<'a>(pub &'a HandleShape);

impl Display for SliderHandleShapeFormat<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            HandleShape::Circle { radius } => write!(f, "SliderHandleShape.circle({:?})", radius),
            HandleShape::Rectangle {
                width,
                border_radius,
            } => write!(
                f,
                "SliderHandleShape.rectangle({:?}, {:?})",
                width, border_radius
            ),
        }
    }
}
