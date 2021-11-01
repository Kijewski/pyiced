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
/// text($module, /, label, *, size=None, color=None, font=None, width=None, height=None, horizontal_alignment=None, vertical_alignment=None)
/// --
///
/// A paragraph of text.
///
/// Parameters
/// ----------
/// label : str
///     The text to display.
/// size : Optional[int]
///     The size of the text.
/// color : Optional[Color]
///     The color of the text.
/// font : Optional[Font]
///     The Font of the text.
/// width : Optional[Length]
///     The width of the text boundaries
/// height : Optional[Length]
///     The height of the text boundaries
/// horizontal_alignment : Optional[HorizontalAlignment]
///     The horizontal alignment of the text.
/// vertical_alignment : Optional[VerticalAlignment]
///     The vertical alignment of the Text
///
/// Returns
/// -------
/// Element
///     The newly created text label.
///
/// See also
/// --------
/// `iced_native::widget::text::Text <https://docs.rs/iced_native/0.4.0/iced_native/widget/text/struct.Text.html>`_
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
    let el = TextBuilder {
        label,
        size,
        color: color.map(|o| o.0),
        font: font.map(|o| o.0),
        width: width.map(|o| o.0),
        height: height.map(|o| o.0),
        horizontal_alignment: horizontal_alignment.map(|o| o.0),
        vertical_alignment: vertical_alignment.map(|o| o.0),
    };
    el.into()
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
