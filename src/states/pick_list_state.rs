use std::sync::Arc;

use iced::pick_list::State;
use parking_lot::RwLock;
use pyo3::prelude::*;

use crate::common::{debug_str, Message};
use crate::make_with_state;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedPickListState>()?;
    Ok(())
}

pub(crate) type PickListState = Arc<RwLock<State<String>>>;

/// PickListState()
/// --
///
/// The state of a :func:`~pyiced.pick_list()`.
#[pyclass(name = "PickListState", module = "pyiced")]
#[derive(Debug, Default, Clone)]
pub(crate) struct WrappedPickListState(pub PickListState);

#[pymethods]
impl WrappedPickListState {
    #[new]
    fn new() -> Self {
        Self::default()
    }

    fn __str__(&self) -> PyResult<String> {
        debug_str(&self.0)
    }
}

make_with_state! {
    pick_list_with_state(
        iced::PickList<String, Message>,
        iced::PickList<'this, String, Message>,
        iced::pick_list::State<String>,
    );
}
