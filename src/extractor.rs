use iced::pane_grid::Line;
use iced::{Background, Color, Vector};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use crate::wrapped::{WrappedColor, WrappedLine, WrappedSliderHandle, WrappedSliderHandleShape};

pub(crate) fn init_mod(_py: Python, _m: &PyModule) -> PyResult<()> {
    Ok(())
}

pub(crate) struct Extractor<'p>(pub &'p PyAny);

impl<'p> TryFrom<Extractor<'p>> for f32 {
    type Error = PyErr;

    fn try_from(value: Extractor<'p>) -> Result<Self, Self::Error> {
        let value = value.0.extract::<f32>()?;
        if !value.is_finite() {
            return Err(PyErr::new::<PyValueError, _>("float values must be finite"));
        }
        Ok(value)
    }
}

impl<'p> TryFrom<Extractor<'p>> for Background {
    type Error = PyErr;

    fn try_from(value: Extractor<'p>) -> Result<Self, Self::Error> {
        value
            .0
            .extract()
            .map(|WrappedColor(c)| Background::Color(c))
    }
}

impl<'p> TryFrom<Extractor<'p>> for Option<Background> {
    type Error = PyErr;

    fn try_from(value: Extractor<'p>) -> Result<Self, Self::Error> {
        value
            .0
            .extract::<Option<_>>()
            .map(|c| c.map(|WrappedColor(c)| Background::Color(c)))
    }
}

impl<'p> TryFrom<Extractor<'p>> for Color {
    type Error = PyErr;

    fn try_from(value: Extractor<'p>) -> Result<Self, Self::Error> {
        value.0.extract().map(|WrappedColor(c)| c)
    }
}

impl<'p> TryFrom<Extractor<'p>> for Vector {
    type Error = PyErr;

    fn try_from(value: Extractor<'p>) -> Result<Self, Self::Error> {
        let (x, y) = value.0.extract::<(f32, f32)>()?;
        if !x.is_finite() || !y.is_finite() {
            return Err(PyErr::new::<PyValueError, _>("float values must be finite"));
        }
        Ok(Vector { x, y })
    }
}

impl<'p> TryFrom<Extractor<'p>> for Option<Line> {
    type Error = PyErr;

    fn try_from(value: Extractor<'p>) -> Result<Self, Self::Error> {
        value
            .0
            .extract::<Option<_>>()
            .map(|c| c.map(|WrappedLine(c)| c))
    }
}

impl<'p> TryFrom<Extractor<'p>> for Option<Color> {
    type Error = PyErr;

    fn try_from(value: Extractor<'p>) -> Result<Self, Self::Error> {
        value
            .0
            .extract::<Option<_>>()
            .map(|c| c.map(|WrappedColor(c)| c))
    }
}

impl<'p> TryFrom<Extractor<'p>> for iced::slider::Handle {
    type Error = PyErr;

    fn try_from(value: Extractor<'p>) -> Result<Self, Self::Error> {
        value.0.extract().map(|WrappedSliderHandle(c)| c)
    }
}

impl<'p> TryFrom<Extractor<'p>> for iced::slider::HandleShape {
    type Error = PyErr;

    fn try_from(value: Extractor<'p>) -> Result<Self, Self::Error> {
        value.0.extract().map(|WrappedSliderHandleShape(c)| c)
    }
}

impl<'p> TryFrom<Extractor<'p>> for (Color, Color) {
    type Error = PyErr;

    fn try_from(value: Extractor<'p>) -> Result<Self, Self::Error> {
        value
            .0
            .extract()
            .map(|(WrappedColor(a), WrappedColor(b))| (a, b))
    }
}
