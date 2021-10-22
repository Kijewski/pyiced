use iced::{Element, Font, Length, TextInput};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::assign;
use crate::common::{to_msg_fn, GCProtocol, Message, ToNative};
use crate::states::{text_input_with_state, TextInputState, WrappedTextInputState};
use crate::widgets::WrappedWidgetBuilder;
use crate::wrapped::{WrappedFont, WrappedLength, WrappedMessage};

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
    pub on_submit: Option<Message>,
    pub password: bool,
    // style: TODO,
}

impl GCProtocol for TextInputBuilder {
    fn traverse(&self, visit: &pyo3::PyVisit) -> Result<(), pyo3::PyTraverseError> {
        visit.call(&self.on_change)?;
        Ok(())
    }
}

#[pyfunction(name = "text_input")]
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
    on_submit: Option<&WrappedMessage>,
    password: bool,
) -> WrappedWidgetBuilder {
    TextInputBuilder {
        state: state.0.clone(),
        placeholder,
        value,
        on_change,
        font: font.map(|o| o.0),
        width: width.map(|o| o.0),
        max_width,
        padding,
        size,
        on_submit: on_submit.map(|o| o.0.clone()),
        password,
    }
    .into()
}

impl ToNative for TextInputBuilder {
    fn to_native(&self, _py: Python) -> Element<'static, Message> {
        text_input_with_state(&self.state, |state| {
            let on_change = to_msg_fn(&self.on_change);
            let el = TextInput::new(state, &self.placeholder, &self.value, on_change);
            let el = assign!(el, self, font, width, max_width, padding, size);
            let el = match &self.on_submit {
                Some(on_submit) => el.on_submit(on_submit.clone()),
                _ => el,
            };
            let el = match self.password {
                true => el.password(),
                false => el,
            };
            Ok(el)
        })
    }
}
