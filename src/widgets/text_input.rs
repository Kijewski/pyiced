use iced::{Element, Font, Length, TextInput};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::assign;
use crate::common::{to_msg_fn, GCProtocol, Message, ToNative};
use crate::states::{text_input_with_state, TextInputState, WrappedTextInputState};
use crate::styles::{TextInputStyleSheet, WrappedTextInputStyleSheet};
use crate::widgets::WrappedWidgetBuilder;
use crate::wrapped::{MessageOrDatum, WrappedFont, WrappedLength};

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(make_text_input, m)?)?;
    Ok(())
}

#[derive(Debug, Clone)]
pub(crate) struct TextInputBuilder {
    pub state: TextInputState,
    pub placeholder: String,
    pub value: String,
    pub on_change: Py<PyAny>, // fn f(value: String) -> crate::Message
    pub font: Option<Font>,
    pub width: Option<Length>,
    pub max_width: Option<u32>,
    pub padding: Option<u16>,
    pub size: Option<u16>,
    pub on_submit: Message,
    pub password: bool,
    pub style: Option<TextInputStyleSheet>,
}

impl GCProtocol for TextInputBuilder {
    fn traverse(&self, visit: &pyo3::PyVisit) -> Result<(), pyo3::PyTraverseError> {
        visit.call(&self.on_change)?;
        self.on_submit.traverse(visit)?;
        Ok(())
    }
}

#[pyfunction(name = "text_input")]
/// text_input($module, /, state, placeholder, value, on_change, *, font=None, width=None, max_width=None, padding=None, size=None, on_submit=None, password=False)
/// --
///
/// A field that can be filled with text.
///
/// Parameters
/// ----------
/// state : TextInputState
///     Current state of the input element. The same object must be given between calls.
/// placeholder : str
///     Placeholder text for an element input.
/// value : str
///     Current value of the input element.
/// on_change : Callable[[str], Optional[object]]
///     Function to call when the text was changed. The new text is the argument of the callback function.
///     The new text should be value for argument "value", but you may reject the new text if it does not fit some criteria defined by you.
///
///     The function can return a message that will be received in the app's :meth:`~pyiced.IcedApp.update` loop.
/// font : Optional[Font]
///     The font of the text.
/// width : Optional[Length]
///     The width of the input element.
/// max_width : Optional[int]
///     The maximum width of the input element.
/// padding : Optional[int]
///     The padding of the input element.
/// size : Optional[int]
///      The text size of the input element.
/// on_submit : Optional[object]
///     Message to send to :meth:`pyiced.IcedApp.update` if :kbd:`Enter` was pressed.
/// password : bool
///     If set to True, the input element becomes a secure password input.
///
/// Returns
/// -------
/// Element
///     The newly created text input element.
///
/// See also
/// --------
/// `iced_native::widget::text_input::TextInput <https://docs.rs/iced_native/0.4.0/iced_native/widget/text_input/struct.TextInput.html>`_
fn make_text_input(
    state: &WrappedTextInputState,
    placeholder: String,
    value: String,
    on_change: Py<PyAny>,
    font: Option<&WrappedFont>,
    width: Option<&WrappedLength>,
    max_width: Option<u32>,
    padding: Option<u16>,
    size: Option<u16>,
    on_submit: Option<MessageOrDatum>,
    password: Option<bool>,
    style: Option<&WrappedTextInputStyleSheet>,
) -> WrappedWidgetBuilder {
    let el = TextInputBuilder {
        state: state.0.clone(),
        placeholder,
        value,
        on_change,
        font: font.map(|o| o.0),
        width: width.map(|o| o.0),
        max_width,
        padding,
        size,
        on_submit: on_submit.unwrap_or_default().into(),
        password: password.unwrap_or(false),
        style: style.map(|o| o.0),
    };
    el.into()
}

impl ToNative for TextInputBuilder {
    fn to_native(&self, _py: Python) -> Element<'static, Message> {
        text_input_with_state(&self.state, |state| {
            let on_change = to_msg_fn(&self.on_change);
            let el = TextInput::new(state, &self.placeholder, &self.value, on_change);
            let el = assign!(el, self, font, width, max_width, padding, size, style);
            let el = match &self.on_submit {
                Message::None => el,
                on_submit => el.on_submit(on_submit.clone()),
            };
            let el = match self.password {
                true => el.password(),
                false => el,
            };
            Ok(el)
        })
    }
}
