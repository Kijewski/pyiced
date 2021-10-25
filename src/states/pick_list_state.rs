use std::sync::Arc;

use iced::pick_list::State;
use parking_lot::Mutex;
use pyo3::prelude::*;
use pyo3::PyObjectProtocol;

use crate::common::{debug_str, Message};
use crate::make_with_state;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedPickListState>()?;
    Ok(())
}

pub(crate) type PickListState = Arc<Mutex<State<String>>>;

/// PickListState()
/// --
///
/// TODO
#[pyclass(name = "PickListState", module = "pyiced")]
#[derive(Debug, Default, Clone)]
pub(crate) struct WrappedPickListState(pub PickListState);

#[pyproto]
impl PyObjectProtocol for WrappedPickListState {
    fn __str__(&self) -> PyResult<String> {
        debug_str(&self.0)
    }
}

#[pymethods]
impl WrappedPickListState {
    /// TODO
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
