use iced::{Element, Length, Radio};
use pyo3::{prelude::*, wrap_pyfunction};

use crate::assign;
use crate::common::{GCProtocol, Message, NonOptional, ToNative, to_msg_fn};
use crate::widgets::WrappedWidgetBuilder;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(make_radio, m)?)?;
    Ok(())
}

#[derive(Debug, Clone)]
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

    fn clear(&mut self) {
        self.f = None;
    }
}

#[pyfunction(name="radio")]
fn make_radio(
) -> WrappedWidgetBuilder {
    todo!()
}

impl ToNative for RadioBuilder {
    fn to_native(&self, _py: Python) -> Element<'static, Message> {
        let f = to_msg_fn(self.f.as_ref().unwrap());
        let el = Radio::new(self.value, &self.label, self.selected, f);
        let el = assign!(el, self, size, width, spacing, text_size);
        el.into()
    }
}
