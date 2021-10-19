use iced::{Checkbox, Element, Font, Length};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::assign;
use crate::common::{empty_space, to_msg_fn, GCProtocol, Message, NonOptional, ToNative};
use crate::widgets::WrappedWidgetBuilder;
use crate::wrapped::{WrappedFont, WrappedLength};

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(make_checkbox, m)?)?;
    Ok(())
}

#[derive(Debug, Clone, Default)]
pub(crate) struct CheckboxBuilder {
    pub is_checked: bool,
    pub label: String,
    pub f: NonOptional<Py<PyAny>>, // fn f(checked: bool) -> crate::Message
    pub size: Option<u16>,
    pub width: Option<Length>,
    pub spacing: Option<u16>,
    pub text_size: Option<u16>,
    pub font: Option<Font>,
    // style: TODO,
}

impl GCProtocol for CheckboxBuilder {
    fn traverse(&self, visit: &pyo3::PyVisit) -> Result<(), pyo3::PyTraverseError> {
        if let Some(f) = &self.f {
            visit.call(f)?;
        }
        Ok(())
    }
}

#[pyfunction(name = "checkbox")]
fn make_checkbox(
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
        width: width.map(|o| o.0),
        spacing,
        text_size,
        font: font.map(|o| o.0),
    }
    .into()
}

impl ToNative for CheckboxBuilder {
    fn to_native(&self, _py: Python) -> Element<'static, Message> {
        let f = match &self.f {
            Some(f) => f,
            None => return empty_space(),
        };
        let f = to_msg_fn(f);
        let el = Checkbox::new(self.is_checked, &self.label, f);
        let el = assign!(el, self, size, width, spacing, text_size, font);
        el.into()
    }
}
