use iced::{Button, Element, Length};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::assign;
use crate::common::{GCProtocol, Message, ToNative};
use crate::states::{button_with_state, ButtonState, WrappedButtonState};
use crate::widgets::{WidgetBuilder, WrappedWidgetBuilder};
use crate::wrapped::{WrappedLength, WrappedMessage};

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
    pub on_press: Option<Message>,
    // style: TODO,
}

impl GCProtocol for ButtonBuilder {
    fn traverse(&self, visit: &pyo3::PyVisit) -> Result<(), pyo3::PyTraverseError> {
        self.content.traverse(visit)
    }
}

#[pyfunction(name = "button")]
/// button($module, /, state, content, *, width=None, height=None, min_width=None, min_height=None, padding=None, on_press=None)
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
/// on_press : Optional[Message]
///     Message to send to the app's :meth:`~pyiced.IcedApp.update` loop when the key was clicked.
///     Without this argument the button won't be clickable.
///
/// Returns
/// -------
/// Element
///     The newly created button.
///
/// See also
/// --------
/// * `iced_native::widget::button::Button <https://docs.rs/iced_native/0.4.0/iced_native/widget/button/struct.Button.html>`_
fn make_button(
    state: &WrappedButtonState,
    content: &WrappedWidgetBuilder,
    width: Option<&WrappedLength>,
    height: Option<&WrappedLength>,
    min_width: Option<u32>,
    min_height: Option<u32>,
    padding: Option<u16>,
    on_press: Option<&WrappedMessage>,
) -> WrappedWidgetBuilder {
    ButtonBuilder {
        state: state.0.clone(),
        content: Box::new(content.0.clone()),
        width: width.map(|o| o.0),
        height: height.map(|o| o.0),
        min_width,
        min_height,
        padding,
        on_press: on_press.map(|o| o.0.clone()),
    }
    .into()
}

impl ToNative for ButtonBuilder {
    fn to_native(&self, py: Python) -> Element<'static, Message> {
        button_with_state(&self.state, |state| {
            let content = self.content.to_native(py);
            let el = Button::new(state, content);
            let el = assign!(el, self, width, height, min_width, min_height, padding);
            let el = match &self.on_press {
                Some(on_press) => el.on_press(on_press.clone()),
                None => el,
            };
            Ok(el)
        })
    }
}
