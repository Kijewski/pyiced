use iced::{Button, Element, Length};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::assign;
use crate::common::{GCProtocol, Message, ToNative};
use crate::states::{button_with_state, ButtonState, WrappedButtonState};
use crate::styles::{ButtonStyle, WrappedButtonStyleSheet};
use crate::widgets::{WidgetBuilder, WrappedWidgetBuilder};
use crate::wrapped::{MessageOrDatum, WrappedLength};

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(make_button, m)?)?;
    Ok(())
}

#[derive(Debug, Clone)]
pub(crate) struct ButtonBuilder {
    pub state: ButtonState,
    pub content: Box<WidgetBuilder>,
    pub width: Option<Length>,
    pub height: Option<Length>,
    pub min_width: Option<u32>,
    pub min_height: Option<u32>,
    pub padding: Option<u16>,
    pub on_press: Message,
    pub style: Option<ButtonStyle>,
}

impl GCProtocol for ButtonBuilder {
    fn traverse(&self, visit: &pyo3::PyVisit) -> Result<(), pyo3::PyTraverseError> {
        self.content.traverse(visit)?;
        self.on_press.traverse(visit)?;
        Ok(())
    }
}

#[pyfunction(name = "button")]
/// button($module, /, state, content, on_press=None, *, width=None, height=None, min_width=None, min_height=None, padding=None, style=None)
/// --
///
/// A generic widget that produces a message when pressed.
///
/// Parameters
/// ----------
/// state : ButtonState
///     Current state of the button. The same object must be given between calls.
/// content : Element
///     The element displayed inside the button, e.g. a :func:`~pyiced.text`.
/// on_press : Optional[object]
///     Message to send to the app's :meth:`~pyiced.IcedApp.update` loop when the key was clicked.
///     Without this argument the button won't be clickable.
/// width : Optional[Length]
///     Width the the button.
/// height : Optional[Length]
///     Height the the button.
/// min_width : Optional[int]
///     Minimum width of the button in pixels.
/// min_height : Optional[int]
///     Minimum height of the button in pixels.
/// padding : Optional[int]
///     Amount of pixels surrounding the contained element.
/// style : Optional[ButtonStyle]
///     The style of the button.
///
/// Returns
/// -------
/// Element
///     The newly created button.
///
/// Example
/// -------
/// .. image:: ../examples/widgets/button.png
///    :width: 688
///    :height: 405
///    :align: center
///    :alt:
///
/// .. literalinclude :: ../examples/widgets/button.py
///    :language: python
///
/// See also
/// --------
/// `iced_native::widget::button::Button <https://docs.rs/iced_native/0.4.0/iced_native/widget/button/struct.Button.html>`_
fn make_button(
    state: &WrappedButtonState,
    content: &WrappedWidgetBuilder,
    on_press: Option<MessageOrDatum>,
    width: Option<&WrappedLength>,
    height: Option<&WrappedLength>,
    min_width: Option<u32>,
    min_height: Option<u32>,
    padding: Option<u16>,
    style: Option<&WrappedButtonStyleSheet>,
) -> WrappedWidgetBuilder {
    let el = ButtonBuilder {
        state: state.0.clone(),
        content: Box::new(content.0.clone()),
        width: width.map(|o| o.0),
        height: height.map(|o| o.0),
        min_width,
        min_height,
        padding,
        on_press: on_press.unwrap_or_default().into(),
        style: style.map(|o| o.0),
    };
    el.into()
}

impl ToNative for ButtonBuilder {
    fn to_native(&self, py: Python) -> Element<'static, Message> {
        button_with_state(&self.state, |state| {
            let content = self.content.to_native(py);
            let el = Button::new(state, content);
            let el = assign!(
                el, self, width, height, min_width, min_height, padding, style,
            );
            let el = match &self.on_press {
                Message::None => el,
                on_press => el.on_press(on_press.clone()),
            };
            Ok(el)
        })
    }
}
