use iced::{Element, Length, Radio};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::assign;
use crate::common::{to_msg_fn, GCProtocol, Message, ToNative};
use crate::widgets::WrappedWidgetBuilder;
use crate::wrapped::WrappedLength;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(make_radio, m)?)?;
    Ok(())
}

#[derive(Debug, Clone)]
pub(crate) struct RadioBuilder {
    pub value: i64,
    pub label: String,
    pub selected: Option<i64>,
    pub f: Py<PyAny>, // fn f(value: i64) -> crate::Message
    pub size: Option<u16>,
    pub width: Option<Length>,
    pub spacing: Option<u16>,
    pub text_size: Option<u16>,
    // style: TODO,
}

impl GCProtocol for RadioBuilder {
    fn traverse(&self, visit: &pyo3::PyVisit) -> Result<(), pyo3::PyTraverseError> {
        visit.call(&self.f)?;
        Ok(())
    }
}

#[pyfunction(name = "radio")]
/// radio($module, /, value, label, selected, f, *, size=None, width=None, spacing=None, text_size=None)
/// --
///
/// Make a .
///
/// Parameters
/// ----------
/// value : int
///     TODO
/// label : String
///     TODO
/// selected : Optional[int]
///     TODO
/// f : Callable[[int], Optional[Message]]
///     TODO
/// size : Optional[int]
///     TODO
/// width : Optional[Length]
///     TODO
/// spacing : Optional[int]
///     TODO
/// text_size : Optional[int]
///     TODO
///
/// Returns
/// -------
/// Element
///     The newly created .
///
/// See also
/// --------
/// * `iced_native::widget::radio::Radio <https://docs.rs/iced_native/0.4.0/iced_native/widget/radio/struct.Radio.html>`_
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
    let el = RadioBuilder {
        value,
        label,
        selected,
        f,
        size,
        width: width.map(|o| o.0),
        spacing,
        text_size,
    };
    el.into()
}

impl ToNative for RadioBuilder {
    fn to_native(&self, _py: Python) -> Element<'static, Message> {
        let f = to_msg_fn(&self.f);
        let el = Radio::new(self.value, self.label.clone(), self.selected, f);
        let el = assign!(el, self, size, width, spacing, text_size);
        el.into()
    }
}
