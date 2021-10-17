use std::sync::Arc;

use parking_lot::Mutex;
use pyo3::{PyObjectProtocol, prelude::*};

use crate::common::{Message, debug_str};
use crate::make_with_state;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedPickListState>()?;
    Ok(())
}

pub(crate) type PickListState = Arc<Mutex<iced::pick_list::State<Message>>>;

#[pyclass(name="PickListState", module="pyiced.pyiced")]
#[derive(Debug, Default)]
pub(crate) struct WrappedPickListState(pub PickListState);

#[pyproto]
impl PyObjectProtocol for WrappedPickListState {
    fn __str__(&self) -> PyResult<String> {
        debug_str(&self.0)
    }
}

#[pymethods]
impl WrappedPickListState {
    #[new]
    fn new() -> Self {
        Self(Arc::new(Mutex::new(Default::default())))
    }
}

make_with_state! {
    pick_list_with_state(
        iced::PickList<String, Message>,
        iced::PickList<'this, String, Message>,
        iced::pick_list::State<String>,
    );
}
