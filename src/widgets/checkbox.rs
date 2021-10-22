use iced::{Checkbox, Element, Font, Length};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::assign;
use crate::common::{to_msg_fn, GCProtocol, Message, ToNative};
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
    pub f: Py<PyAny>, // fn f(checked: bool) -> crate::Message
    pub size: Option<u16>,
    pub width: Option<Length>,
    pub spacing: Option<u16>,
    pub text_size: Option<u16>,
    pub font: Option<Font>,
    // style: TODO,
}

impl GCProtocol for CheckboxBuilder {
    fn traverse(&self, visit: &pyo3::PyVisit) -> Result<(), pyo3::PyTraverseError> {
        visit.call(&self.f)?;
        Ok(())
    }
}

#[pyfunction(name = "checkbox")]
/// checkbox($module, /, is_checked, label, *, f=None, size=None, width=None, spacing=None, text_size=None, font=None)
/// --
///
/// Make a checkbox.
///
/// Parameters
/// ----------
/// is_checked : bool
///     Whether the checkbox is currently checked or not.
/// label : str
///     A text besides the checkbox. Might be empty.
/// f : Optional[Callable[[bool], Optional[Message]]]
///     Function to call when the checkbox is toggled. The argument is the new checked state.
///     The function can return a message that will be received in the app's :meth:`~pyiced.IcedApp.update` loop.
/// size : Optional[int]
///     Size of the checkbox.
/// width : Optional[Length]
///     Width of the widget (checkbox and text).
/// spacing : Optional[int]
///     Space between checkbox and text.
/// text_size : Optional[int]
///     Font size of the text.
/// font : Optional[Font]
///     Font of the text.
///
/// Returns
/// -------
/// Element
///     Newly created checkbox.
/// See also
/// --------
/// * `iced_native::widget::checkbox::Checkbox <https://docs.rs/iced_native/0.4.0/iced_native/widget/checkbox/struct.Checkbox.html>`_
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
        f,
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
        let f = to_msg_fn(&self.f);
        let el = Checkbox::new(self.is_checked, &self.label, f);
        let el = assign!(el, self, size, width, spacing, text_size, font);
        el.into()
    }
}
