use pyo3::{prelude::*, wrap_pyfunction};

use crate::common::{Message, NonOptional, ToNative};
use crate::widgets::WrappedWidgetBuilder;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(make_pick_list, m)?)?;
    Ok(())
}

#[derive(Debug, Clone)]
pub(crate) struct PickListBuilder {
    pub state: iced::pick_list::State<String>,
    pub options: Vec<String>,
    pub selected: Option<String>,
    pub on_selected: NonOptional<Py<PyAny>>, // fn on_selected(value: String) -> crate::Message
}

#[pyfunction(name="pick_list")]
fn make_pick_list<'p>(
) -> WrappedWidgetBuilder {
    todo!()
}

impl ToNative for PickListBuilder {
    fn to_native(&self, _py: Python) -> iced::Element<'static, Message> {
        todo!();
        // let on_select = to_msg_fn(&self.on_selected.unwrap());
        // let el = iced::PickList::new(&mut self.state, &self.options[..], self.selected.clone(), on_select);
        // el.into()
    }
}
