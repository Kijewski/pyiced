use iced::pane_grid::Line;
use iced::rule::FillMode;
use iced::scrollable::Scroller;
use iced::{Background, Color, Vector};
use pyo3::prelude::*;

use crate::common::validate_f32;
use crate::styles::{
    CheckboxStyle, PickListMenu, PickListStyle, RadioStyle, ScrollbarStyle, ScrollerStyle,
    TextInputStyle, WrappedCheckboxStyle, WrappedPickListMenu, WrappedPickListStyle,
    WrappedRadioStyle, WrappedScrollbarStyle, WrappedScrollerStyle, WrappedTextInputStyle,
};
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

pub(crate) struct Unextractor<'a, Src>(pub &'a Src);

pub(crate) trait Unextract<Dest> {
    fn unextract(self) -> Dest;
}

impl Unextract<WrappedColor> for Unextractor<'_, Color> {
    fn unextract(self) -> WrappedColor {
        WrappedColor(*self.0)
    }
}

impl Unextract<Option<WrappedColor>> for Unextractor<'_, Option<Color>> {
    fn unextract(self) -> Option<WrappedColor> {
        self.0.map(WrappedColor)
    }
}

impl Unextract<WrappedColor> for Unextractor<'_, Background> {
    fn unextract(self) -> WrappedColor {
        match self.0 {
            &Background::Color(color) => WrappedColor(color),
        }
    }
}

impl Unextract<Option<WrappedColor>> for Unextractor<'_, Option<Background>> {
    fn unextract(self) -> Option<WrappedColor> {
        self.0.map(|Background::Color(color)| WrappedColor(color))
    }
}

impl<'a, T: Clone> Unextract<T> for Unextractor<'a, T> {
    fn unextract(self) -> T {
        self.0.clone()
    }
}

impl<T: Copy> Unextract<(T, T)> for Unextractor<'_, Vector<T>> {
    fn unextract(self) -> (T, T) {
        let &Vector { x, y } = self.0;
        (x, y)
    }
}

impl Unextract<WrappedCheckboxStyle> for Unextractor<'_, iced::checkbox::Style> {
    fn unextract(self) -> WrappedCheckboxStyle {
        WrappedCheckboxStyle(CheckboxStyle(*self.0))
    }
}

impl Unextract<Option<WrappedLine>> for Unextractor<'_, Option<Line>> {
    fn unextract(self) -> Option<WrappedLine> {
        self.0.map(WrappedLine)
    }
}

impl Unextract<WrappedPickListStyle> for Unextractor<'_, iced::pick_list::Style> {
    fn unextract(self) -> WrappedPickListStyle {
        WrappedPickListStyle(PickListStyle(*self.0))
    }
}

impl Unextract<WrappedPickListMenu> for Unextractor<'_, iced::pick_list::Menu> {
    fn unextract(self) -> WrappedPickListMenu {
        WrappedPickListMenu(PickListMenu(*self.0))
    }
}

impl Unextract<WrappedRadioStyle> for Unextractor<'_, iced::radio::Style> {
    fn unextract(self) -> WrappedRadioStyle {
        WrappedRadioStyle(RadioStyle(*self.0))
    }
}

impl Unextract<WrappedFillMode> for Unextractor<'_, FillMode> {
    fn unextract(self) -> WrappedFillMode {
        WrappedFillMode(*self.0)
    }
}

impl Unextract<WrappedScrollerStyle> for Unextractor<'_, iced::scrollable::Scroller> {
    fn unextract(self) -> WrappedScrollerStyle {
        WrappedScrollerStyle(ScrollerStyle(*self.0))
    }
}

impl Unextract<WrappedScrollbarStyle> for Unextractor<'_, iced::scrollable::Scrollbar> {
    fn unextract(self) -> WrappedScrollbarStyle {
        WrappedScrollbarStyle(ScrollbarStyle(*self.0))
    }
}

impl Unextract<(WrappedColor, WrappedColor)> for Unextractor<'_, (Color, Color)> {
    fn unextract(self) -> (WrappedColor, WrappedColor) {
        let (a, b) = *self.0;
        (WrappedColor(a), WrappedColor(b))
    }
}

impl Unextract<WrappedSliderHandle> for Unextractor<'_, iced::slider::Handle> {
    fn unextract(self) -> WrappedSliderHandle {
        WrappedSliderHandle(*self.0)
    }
}

impl Unextract<WrappedTextInputStyle> for Unextractor<'_, iced::text_input::Style> {
    fn unextract(self) -> WrappedTextInputStyle {
        WrappedTextInputStyle(TextInputStyle(*self.0))
    }
}
