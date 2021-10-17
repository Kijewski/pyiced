use pyo3::{prelude::*, wrap_pyfunction};

use crate::common::{Message, NonOptional, ToNative};
use crate::widgets::WrappedWidgetBuilder;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(make_text_input, m)?)?;
    Ok(())
}

#[derive(Debug, Clone)]
pub(crate) struct TextInputBuilder {
    pub state: iced::text_input::State,
    pub placeholder: String,
    pub value: String,
    pub on_change: NonOptional<Py<PyAny>>, // fn f(value: String) -> crate::Message
    pub font: Option<iced::Font>,
    pub width: Option<iced::Length>,
    pub max_width: Option<u32>,
    pub padding: Option<u16>,
    pub size: Option<u16>,
    pub on_submit: Option<Message>,
    pub password: bool,
    // style: TODO,
}

#[pyfunction(name="text_input")]
fn make_text_input<'p>(
) -> WrappedWidgetBuilder {
    todo!()
}

impl ToNative for TextInputBuilder {
    fn to_native(&self, _py: Python) -> iced::Element<'static, Message> {
        todo!();
        // let on_change = to_msg_fn(&self.on_change.unwrap());
        // let el = iced::TextInput::new(&mut self.state, &self.placeholder, &self.value, on_change);
        // let el = assign!(el, self, font, width, max_width, padding, size);
        // let el = match &self.on_submit {
        //     Some(on_submit) => el.on_submit(on_submit.clone()),
        //     _ => el,
        // };
        // let el = match self.password {
        //     true => el.password(),
        //     false => el,
        // };
        // el.into()
    }
}
