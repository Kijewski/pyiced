use iced::pane_grid::Line;
use iced::rule::FillMode;
use iced::scrollable::Scroller;
use iced::{Background, Color, Vector};
use pyo3::prelude::*;

use crate::common::validate_f32;
use crate::styles::WrappedScrollerStyle;
use crate::wrapped::{
    WrappedColor, WrappedFillMode, WrappedLine, WrappedSliderHandle, WrappedSliderHandleShape,
};

pub(crate) fn init_mod(_py: Python, _m: &PyModule) -> PyResult<()> {
    Ok(())
}

pub(crate) struct Extractor<'p>(pub &'p PyAny);

impl<'p> TryFrom<Extractor<'p>> for FillMode {
    type Error = PyErr;

    fn try_from(value: Extractor<'p>) -> PyResult<FillMode> {
        Ok(value.0.extract::<WrappedFillMode>()?.0)
    }
}

impl<'p> TryFrom<Extractor<'p>> for u16 {
    type Error = PyErr;

    fn try_from(value: Extractor<'p>) -> PyResult<u16> {
        value.0.extract::<u16>()
    }
}

impl<'p> TryFrom<Extractor<'p>> for f32 {
    type Error = PyErr;

    fn try_from(value: Extractor<'p>) -> PyResult<f32> {
        validate_f32(value.0.extract::<f32>()?)
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
        Ok(Vector {
            x: validate_f32(x)?,
            y: validate_f32(y)?,
        })
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

impl<'p> TryFrom<Extractor<'p>> for Scroller {
    type Error = PyErr;

    fn try_from(value: Extractor<'p>) -> Result<Self, Self::Error> {
        value.0.extract().map(|WrappedScrollerStyle(a)| a.0)
    }
}
