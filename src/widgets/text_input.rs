use iced::{Element, Font, Length, TextInput};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::assign;
use crate::common::{GCProtocol, Message, ToNative};
use crate::states::{text_input_with_state, TextInputState, WrappedTextInputState};
use crate::styles::{TextInputStyleSheet, WrappedTextInputStyleSheet};
use crate::widgets::WrappedWidgetBuilder;
use crate::wrapped::{WrappedFont, WrappedLength};

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(make_text_input, m)?)?;
    Ok(())
}

#[derive(Debug, Clone)]
pub(crate) struct TextInputBuilder {
    pub state: TextInputState,
    pub token: Py<PyAny>,
    pub placeholder: String,
    pub value: String,
    pub font: Option<Font>,
    pub width: Option<Length>,
    pub max_width: Option<u32>,
    pub padding: Option<u16>,
    pub size: Option<u16>,
    pub password: bool,
    pub style: Option<TextInputStyleSheet>,
}

impl GCProtocol for TextInputBuilder {
    fn traverse(&self, visit: &pyo3::PyVisit) -> Result<(), pyo3::PyTraverseError> {
        visit.call(&self.token)?;
        Ok(())
    }
}

#[pyfunction(name = "text_input")]
/// text_input($module, /, token, state, placeholder, value, *, font=None, width=None, max_width=None, padding=None, size=None, password=False, style=None)
/// --
///
/// A field that can be filled with text.
///
/// Parameters
/// ----------
/// token : object
///     When the user changes the text, a message ``(token, new_value)`` is sent to the app's :meth:`~pyiced.IcedApp.update()` method.
///
///     When the user hits enter, a message ``(token, None, 'submit')`` is sent.
/// state : TextInputState
///     Current state of the input element. The same object must be given between calls.
/// placeholder : str
///     Placeholder text for an element input.
/// value : str
///     Current value of the input element.
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
/// password : bool
///     If set to True, the input element becomes a secure password input.
/// style : Optional[TextInputStyleSheet]
///     Style of the text input.
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
    token: Py<PyAny>,
    state: &WrappedTextInputState,
    placeholder: String,
    value: String,
    font: Option<&WrappedFont>,
    width: Option<&WrappedLength>,
    max_width: Option<u32>,
    padding: Option<u16>,
    size: Option<u16>,
    password: Option<bool>,
    style: Option<&WrappedTextInputStyleSheet>,
) -> WrappedWidgetBuilder {
    let el = TextInputBuilder {
        state: state.0.clone(),
        placeholder,
        value,
        token,
        font: font.map(|o| o.0),
        width: width.map(|o| o.0),
        max_width,
        padding,
        size,
        password: password.unwrap_or(false),
        style: style.map(|o| o.0),
    };
    el.into()
}

impl ToNative for TextInputBuilder {
    fn to_native(&self, py: Python) -> Element<'static, Message> {
        let on_submit = Message::Python((self.token.clone(), (), "submit").into_py(py));

        let token = self.token.clone();
        let on_change =
            move |s| Python::with_gil(|py| Message::Python((token.clone(), &s).into_py(py)));

        text_input_with_state(&self.state, move |state| {
            let el = TextInput::new(state, &self.placeholder, &self.value, on_change);
            let el = assign!(el, self, font, width, max_width, padding, size, style);
            let el = el.on_submit(on_submit);
            let el = match self.password {
                true => el.password(),
                false => el,
            };
            Ok(el)
        })
    }
}
