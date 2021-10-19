use std::borrow::Cow;

use iced::{Element, PickList};
use pyo3::prelude::*;
use pyo3::types::PyList;
use pyo3::wrap_pyfunction;

use crate::common::{empty_space, to_msg_fn, GCProtocol, Message, NonOptional, ToNative};
use crate::states::{pick_list_with_state, PickListState, WrappedPickListState};
use crate::widgets::WrappedWidgetBuilder;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(make_pick_list, m)?)?;
    Ok(())
}

#[derive(Debug, Clone, Default)]
pub(crate) struct PickListBuilder {
    pub state: NonOptional<PickListState>,
    pub options: Vec<String>,
    pub selected: Option<String>,
    pub on_selected: NonOptional<Py<PyAny>>, // fn on_selected(value: String) -> crate::Message
}

impl GCProtocol for PickListBuilder {
    fn traverse(&self, visit: &pyo3::PyVisit) -> Result<(), pyo3::PyTraverseError> {
        if let Some(on_selected) = &self.on_selected {
            visit.call(on_selected)?;
        }
        Ok(())
    }
}

#[pyfunction(name = "pick_list")]
fn make_pick_list(
    py: Python,
    state: &WrappedPickListState,
    options: &PyList,
    selected: Option<String>,
    on_selected: Py<PyAny>,
) -> WrappedWidgetBuilder {
    let options = options
        .iter()
        .filter_map(|child| match child.str() {
            Ok(s) => Some(s.to_string()),
            Err(err) => {
                err.print(py);
                None
            },
        })
        .collect();
    PickListBuilder {
        state: Some(state.0.clone()),
        options,
        selected,
        on_selected: Some(on_selected),
    }
    .into()
}

impl ToNative for PickListBuilder {
    fn to_native(&self, _py: Python) -> Element<'static, Message> {
        let on_selected = match &self.on_selected {
            Some(on_selected) => to_msg_fn(on_selected),
            None => return empty_space(),
        };
        pick_list_with_state(self.state.as_ref(), |state| {
            let options = Cow::Owned(self.options.clone());
            let el = PickList::new(state, options, self.selected.clone(), on_selected);
            Ok(el)
        })
    }
}
