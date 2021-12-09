use std::borrow::Cow;

use iced::{Element, Font, PickList};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::assign;
use crate::common::{GCProtocol, Message, ToNative};
use crate::states::{pick_list_with_state, PickListState, WrappedPickListState};
use crate::styles::{PickListStyleSheet, WrappedPickListStyleSheet};
use crate::widgets::WrappedWidgetBuilder;
use crate::wrapped::WrappedFont;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(make_pick_list, m)?)?;
    Ok(())
}

#[derive(Debug, Clone)]
pub(crate) struct PickListBuilder {
    pub state: PickListState,
    pub options: Vec<String>,
    pub selected: Option<String>,
    pub token: Py<PyAny>,
    pub text_size: Option<u16>,
    pub font: Option<Font>,
    pub style: Option<PickListStyleSheet>,
}

impl GCProtocol for PickListBuilder {
    fn traverse(&self, visit: &pyo3::PyVisit) -> Result<(), pyo3::PyTraverseError> {
        visit.call(&self.token)?;
        Ok(())
    }
}

#[pyfunction(name = "pick_list")]
/// pick_list($module, /, token, state, selected, options, *, text_size=None, font=None, style=None)
/// --
///
/// A widget for selecting a single value from a list of options.
///
/// Parameters
/// ----------
/// token : object
///     When the user select a value, a message ``(token, new_value)`` is sent to the app's :meth:`~pyiced.IcedApp.update()` method.
/// state : PickListState
///     Current state of the pick list. The same object must be given between calls.
/// selected : Optional[str]
///     The currently selected value.
/// options : Iterable[Optional[str]]
///     Values to select from.
/// text_size : Optional[int]
///     The text size of the pick list.
/// font : Optional[Font]
///     Font of the pick list.
/// style : Optional[PickListStyle]
///     Style of the pick list.
///
/// Returns
/// -------
/// Element
///     The newly created pick list.
///
/// Example
/// -------
/// .. image:: ../examples/widgets/pick_list.png
///    :align: center
///    :alt:
///
/// .. literalinclude :: ../examples/widgets/pick_list.py
///    :language: python
///
/// See also
/// --------
/// `iced_native::widget::pick_list::PickList <https://docs.rs/iced_native/0.4.0/iced_native/widget/pick_list/struct.PickList.html>`_
fn make_pick_list(
    py: Python,
    token: Py<PyAny>,
    state: &WrappedPickListState,
    selected: Option<String>,
    options: &PyAny,
    text_size: Option<u16>,
    font: Option<&WrappedFont>,
    style: Option<&WrappedPickListStyleSheet>,
) -> PyResult<WrappedWidgetBuilder> {
    let options = options
        .iter()?
        .filter_map(|child| match child {
            Ok(child) if !child.is_none() => match child.str() {
                Ok(s) => Some(s.to_string()),
                Err(err) => {
                    err.print(py);
                    None
                },
            },
            Ok(_) => None,
            Err(err) => {
                err.print(py);
                None
            },
        })
        .collect();
    let el = PickListBuilder {
        state: state.0.clone(),
        options,
        selected,
        token,
        text_size,
        font: font.map(|o| o.0),
        style: style.map(|o| o.0),
    };
    Ok(el.into())
}

impl ToNative for PickListBuilder {
    fn to_native(&self, _py: Python) -> Element<'static, Message> {
        let token = self.token.clone();
        let on_selected = move |value| {
            Python::with_gil(|py| Message::Python((token.clone(), &value).into_py(py)))
        };
        pick_list_with_state(&self.state, move |state| {
            let options = Cow::Owned(self.options.clone());
            let el = PickList::new(state, options, self.selected.clone(), on_selected);
            let el = assign!(el, self, text_size, font, style);
            Ok(el)
        })
    }
}
