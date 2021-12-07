use iced::{Element, Length, Radio};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::assign;
use crate::common::{GCProtocol, Message, ToNative};
use crate::styles::{RadioStyleSheet, WrappedRadioStyleSheet};
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
    pub token: Py<PyAny>,
    pub size: Option<u16>,
    pub width: Option<Length>,
    pub spacing: Option<u16>,
    pub text_size: Option<u16>,
    pub style: Option<RadioStyleSheet>,
}

impl GCProtocol for RadioBuilder {
    fn traverse(&self, visit: &pyo3::PyVisit) -> Result<(), pyo3::PyTraverseError> {
        visit.call(&self.token)?;
        Ok(())
    }
}

#[pyfunction(name = "radio")]
/// radio($module, /, selected, token, value, label, *, size=None, width=None, spacing=None, text_size=None)
/// --
///
/// A circular button representing a choice.
///
/// Parameters
/// ----------
/// selected : Optional[int]
///     The identifier of the currently selected option.
/// token : object
///     When the user select this choice, a message ``(token, value)`` is sent to the app's :meth:`~pyiced.IcedApp.update()` method.
/// value : int
///     Identifier of the option.
/// label : str
///     Label next to the radio button.
/// size : Optional[int]
///     The diameter of the circle.
/// width : Optional[Length]
///     The width including the text.
/// spacing : Optional[int]
///     The spacing between the radio button and its text.
/// text_size : Optional[int]
///     The size of the text.
/// style : Optional[RadioStyleSheet]
///     Style of the radio button.
///
/// Returns
/// -------
/// Element
///     The newly created radio button.
///
/// Example
/// -------
/// .. image:: ../examples/widgets/radio.png
///    :align: center
///    :alt:
///
/// .. literalinclude :: ../examples/widgets/radio.py
///    :language: python
///
/// See also
/// --------
/// `iced_native::widget::radio::Radio <https://docs.rs/iced_native/0.4.0/iced_native/widget/radio/struct.Radio.html>`_
fn make_radio(
    selected: Option<i64>,
    token: Py<PyAny>,
    value: i64,
    label: String,
    size: Option<u16>,
    width: Option<&WrappedLength>,
    spacing: Option<u16>,
    text_size: Option<u16>,
    style: Option<&WrappedRadioStyleSheet>,
) -> WrappedWidgetBuilder {
    let el = RadioBuilder {
        value,
        label,
        selected,
        token,
        size,
        width: width.map(|o| o.0),
        spacing,
        text_size,
        style: style.map(|o| o.0),
    };
    el.into()
}

impl ToNative for RadioBuilder {
    fn to_native(&self, py: Python) -> Element<'static, Message> {
        let msg: Py<PyAny> = (&self.token, self.value).into_py(py);
        let f = move |_| Message::Python(msg.clone());
        let el = Radio::new(self.value, self.label.clone(), self.selected, f);
        let el = assign!(el, self, size, width, spacing, text_size, style);
        el.into()
    }
}
