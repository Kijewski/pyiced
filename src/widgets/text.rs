use iced::{Color, Element, Font, HorizontalAlignment, Length, Text, VerticalAlignment};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::assign;
use crate::common::{GCProtocol, Message, ToNative};
use crate::widgets::WrappedWidgetBuilder;
use crate::wrapped::{
    WrappedColor, WrappedFont, WrappedHorizontalAlignment, WrappedLength, WrappedVerticalAlignment,
};

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(make_text, m)?)?;
    Ok(())
}

#[derive(Debug, Clone, Default)]
pub(crate) struct TextBuilder {
    pub label: String,
    pub size: Option<u16>,
    pub color: Option<Color>,
    pub font: Option<Font>,
    pub width: Option<Length>,
    pub height: Option<Length>,
    pub horizontal_alignment: Option<HorizontalAlignment>,
    pub vertical_alignment: Option<VerticalAlignment>,
}

impl GCProtocol for TextBuilder {}

#[pyfunction(name = "text")]
fn make_text(
    label: String,
    size: Option<u16>,
    color: Option<&WrappedColor>,
    font: Option<&WrappedFont>,
    width: Option<&WrappedLength>,
    height: Option<&WrappedLength>,
    horizontal_alignment: Option<&WrappedHorizontalAlignment>,
    vertical_alignment: Option<&WrappedVerticalAlignment>,
) -> WrappedWidgetBuilder {
    TextBuilder {
        label,
        size,
        color: color.map(|o| o.0),
        font: font.map(|o| o.0),
        width: width.map(|o| o.0),
        height: height.map(|o| o.0),
        horizontal_alignment: horizontal_alignment.map(|o| o.0),
        vertical_alignment: vertical_alignment.map(|o| o.0),
    }
    .into()
}

impl ToNative for TextBuilder {
    fn to_native(&self, _py: Python) -> Element<'static, Message> {
        let el = Text::new(&self.label);
        let el = assign!(
            el,
            self,
            size,
            color,
            font,
            width,
            height,
            horizontal_alignment,
            vertical_alignment
        );
        el.into()
    }
}
