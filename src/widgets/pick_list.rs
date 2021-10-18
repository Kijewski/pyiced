use pyo3::{prelude::*, wrap_pyfunction, types::PyList};

use crate::common::{GCProtocol, Message, NonOptional, ToNative};
use crate::states::{PickListState, WrappedPickListState};
use crate::widgets::WrappedWidgetBuilder;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(make_pick_list, m)?)?;
    Ok(())
}

#[derive(Debug, Clone)]
pub(crate) struct PickListBuilder {
    pub state: NonOptional<PickListState>,
    pub options: Vec<String>,
    pub selected: Option<String>,
    pub on_selected: NonOptional<Py<PyAny>>, // fn on_selected(value: String) -> crate::Message
}

impl GCProtocol for PickListBuilder {
    fn traverse(&self, visit: &pyo3::PyVisit) -> Result<(), pyo3::PyTraverseError> {
        if let Some(on_selected) = &self.on_selected  {
            visit.call(on_selected)?;
        }
        Ok(())
    }

    fn clear(&mut self) {
        self.on_selected = None;
    }
}

#[pyfunction(name="pick_list")]
fn make_pick_list(
    py: Python,
    state: &WrappedPickListState,
    options: &PyList,
    selected: Option<String>,
    on_selected: Py<PyAny>,
) -> WrappedWidgetBuilder {
    let options = options.iter()
        .filter_map(|child| match child.str() {
            Ok(s) => Some(s.to_string()),
            Err(err) => {
                err.print(py);
                None
            }
        })
        .collect();
    PickListBuilder {
        state: Some(state.0.clone()),
        options,
        selected,
        on_selected: Some(on_selected),
    }.into()
}

impl ToNative for PickListBuilder {
    fn to_native(&self, _py: Python) -> iced::Element<'static, Message> {
        todo!();
        // let on_select = to_msg_fn(&self.on_selected.unwrap());
        // let el = iced::PickList::new(&mut self.state, &self.options[..], self.selected.clone(), on_select);
        // el.into()
    }
}
