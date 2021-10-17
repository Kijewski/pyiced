use std::borrow::Cow;

use iced::{Element, Font, PickList};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::assign;
use crate::common::{to_msg_fn, GCProtocol, Message, ToNative};
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
    pub on_selected: Py<PyAny>,
    pub text_size: Option<u16>,
    pub font: Option<Font>,
    pub style: Option<PickListStyleSheet>,
}

impl GCProtocol for PickListBuilder {
    fn traverse(&self, visit: &pyo3::PyVisit) -> Result<(), pyo3::PyTraverseError> {
        visit.call(&self.on_selected)?;
        Ok(())
    }
}

#[pyfunction(name = "pick_list")]
/// pick_list($module, /, state, options, selected, on_selected, *, text_size=None, font=None, style=None)
/// --
///
/// A widget for selecting a single value from a list of options.
///
/// Parameters
/// ----------
/// state : PickListState
///     Current state of the pick list. The same object must be given between calls.
/// options : Iterable[Optional[str]]
///     Values to select from.
/// selected : Optional[str]
///     The currently selected value.
/// on_selected : Callable[[str], Optional[object]]
///     Function to call when a new value was selected.
///
///     The function can return a message that will be received in the app's :meth:`~pyiced.IcedApp.update` loop.
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
///    :width: 688
///    :height: 405
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
    state: &WrappedPickListState,
    options: &PyAny,
    selected: Option<String>,
    on_selected: Py<PyAny>,
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
        on_selected,
        text_size,
        font: font.map(|o| o.0),
        style: style.map(|o| o.0),
    };
    Ok(el.into())
}

impl ToNative for PickListBuilder {
    fn to_native(&self, _py: Python) -> Element<'static, Message> {
        pick_list_with_state(&self.state, |state| {
            let on_selected = to_msg_fn(&self.on_selected);
            let options = Cow::Owned(self.options.clone());
            let el = PickList::new(state, options, self.selected.clone(), on_selected);
            let el = assign!(el, self, text_size, font, style);
            Ok(el)
        })
    }
}
