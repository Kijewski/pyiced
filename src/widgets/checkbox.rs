use pyo3::{prelude::*, wrap_pyfunction};

use crate::assign;
use crate::common::{Message, NonOptional, ToNative, to_msg_fn};
use crate::widgets::WrappedWidgetBuilder;
use crate::wrapped::{WrappedFont, WrappedLength};

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(make_checkbox, m)?)?;
    Ok(())
}

#[derive(Debug, Clone)]
pub(crate) struct CheckboxBuilder {
    pub is_checked: bool,
    pub label: String,
    pub f: NonOptional<Py<PyAny>>, // fn f(checked: bool) -> crate::Message
    pub size: Option<u16>,
    pub width: Option<iced::Length>,
    pub spacing: Option<u16>,
    pub text_size: Option<u16>,
    pub font: Option<iced::Font>,
    // style: TODO,
}

#[pyfunction(name="checkbox")]
fn make_checkbox<'p>(
    is_checked: bool,
    label: String,
    f: Py<PyAny>,
    size: Option<u16>,
    width: Option<&WrappedLength>,
    spacing: Option<u16>,
    text_size: Option<u16>,
    font: Option<&WrappedFont>,
) -> WrappedWidgetBuilder {
    CheckboxBuilder {
        is_checked,
        label,
        f: Some(f),
        size,
        width: width.map(|o| o.0.clone()),
        spacing,
        text_size,
        font: font.map(|o| o.0.clone()),
    }.into()
}

impl ToNative for CheckboxBuilder {
    fn to_native(&self, _py: Python) -> iced::Element<'static, Message> {
        let f = to_msg_fn(self.f.as_ref().unwrap());
        let el = iced::Checkbox::new(self.is_checked, &self.label, f);
        let el = assign!(el, self, size, width, spacing, text_size, font);
        el.into()
    }
}
