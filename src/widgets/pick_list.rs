use std::borrow::Cow;

use iced::{Element, PickList};
use pyo3::prelude::*;
use pyo3::types::PyList;
use pyo3::wrap_pyfunction;

use crate::common::{to_msg_fn, GCProtocol, Message, ToNative};
use crate::states::{pick_list_with_state, PickListState, WrappedPickListState};
use crate::widgets::WrappedWidgetBuilder;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(make_pick_list, m)?)?;
    Ok(())
}

#[derive(Debug, Clone)]
pub(crate) struct PickListBuilder {
    pub state: PickListState,
    pub options: Vec<String>,
    pub selected: Option<String>,
    pub on_selected: Py<PyAny>, // fn on_selected(value: String) -> crate::Message
}

impl GCProtocol for PickListBuilder {
    fn traverse(&self, visit: &pyo3::PyVisit) -> Result<(), pyo3::PyTraverseError> {
        visit.call(&self.on_selected)?;
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
        state: state.0.clone(),
        options,
        selected,
        on_selected,
    }
    .into()
}

impl ToNative for PickListBuilder {
    fn to_native(&self, _py: Python) -> Element<'static, Message> {
        pick_list_with_state(&self.state, |state| {
            let on_selected = to_msg_fn(&self.on_selected);
            let options = Cow::Owned(self.options.clone());
            let el = PickList::new(state, options, self.selected.clone(), on_selected);
            Ok(el)
        })
    }
}
