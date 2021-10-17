use pyo3::{prelude::*, wrap_pyfunction};

use crate::assign;
use crate::common::{Message, ToNative};
use crate::wrapped::{WrappedColor, WrappedFont, WrappedLength, WrappedHorizontalAlignment, WrappedVerticalAlignment};
use crate::widgets::WrappedWidgetBuilder;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(make_text, m)?)?;
    Ok(())
}

#[derive(Debug, Clone)]
pub(crate) struct TextBuilder {
    pub label: String,
    pub size: Option<u16>,
    pub color: Option<iced::Color>,
    pub font: Option<iced::Font>,
    pub width: Option<iced::Length>,
    pub height: Option<iced::Length>,
    pub horizontal_alignment: Option<iced::HorizontalAlignment>,
    pub vertical_alignment: Option<iced::VerticalAlignment>,
}

#[pyfunction(name="text")]
fn make_text<'p>(
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
        color: color.map(|o| o.0.clone()),
        font: font.map(|o| o.0.clone()),
        width: width.map(|o| o.0.clone()),
        height: height.map(|o| o.0.clone()),
        horizontal_alignment: horizontal_alignment.map(|o| o.0.clone()),
        vertical_alignment: vertical_alignment.map(|o| o.0.clone()),
    }.into()
}

impl ToNative for TextBuilder {
    fn to_native(&self, _py: Python) -> iced::Element<'static, Message> {
        let el = iced::Text::new(&self.label);
        let el = assign!(el, self, size, color, font, width, height, horizontal_alignment, vertical_alignment);
        el.into()
    }
}
