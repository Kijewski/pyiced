use iced::{Element, Length, Radio};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::assign;
use crate::common::{empty_space, to_msg_fn, GCProtocol, Message, NonOptional, ToNative};
use crate::widgets::WrappedWidgetBuilder;
use crate::wrapped::WrappedLength;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(make_radio, m)?)?;
    Ok(())
}

#[derive(Debug, Clone, Default)]
pub(crate) struct RadioBuilder {
    pub value: i64,
    pub label: String,
    pub selected: Option<i64>,
    pub f: NonOptional<Py<PyAny>>, // fn f(value: i64) -> crate::Message
    pub size: Option<u16>,
    pub width: Option<Length>,
    pub spacing: Option<u16>,
    pub text_size: Option<u16>,
    // style: TODO,
}

impl GCProtocol for RadioBuilder {
    fn traverse(&self, visit: &pyo3::PyVisit) -> Result<(), pyo3::PyTraverseError> {
        if let Some(f) = &self.f {
            visit.call(f)?;
        }
        Ok(())
    }
}

#[pyfunction(name = "radio")]
fn make_radio(
    value: i64,
    label: String,
    selected: Option<i64>,
    f: Py<PyAny>,
    size: Option<u16>,
    width: Option<&WrappedLength>,
    spacing: Option<u16>,
    text_size: Option<u16>,
) -> WrappedWidgetBuilder {
    RadioBuilder {
        value,
        label,
        selected,
        f: Some(f),
        size,
        width: width.map(|o| o.0),
        spacing,
        text_size,
    }
    .into()
}

impl ToNative for RadioBuilder {
    fn to_native(&self, _py: Python) -> Element<'static, Message> {
        let f = match &self.f {
            Some(f) => f,
            None => return empty_space(),
        };
        let f = to_msg_fn(f);
        let el = Radio::new(self.value, self.label.clone(), self.selected, f);
        let el = assign!(el, self, size, width, spacing, text_size);
        el.into()
    }
}
