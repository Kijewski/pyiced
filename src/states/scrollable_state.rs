use std::sync::Arc;

use iced::scrollable::State;
use parking_lot::Mutex;
use pyo3::prelude::*;
use pyo3::PyObjectProtocol;

use crate::common::debug_str;
use crate::make_with_state;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedScrollableState>()?;
    Ok(())
}

pub(crate) type ScrollableState = Arc<Mutex<State>>;

#[pyclass(name = "ScrollableState", module = "pyiced")]
#[derive(Debug, Default, Clone)]
pub(crate) struct WrappedScrollableState(pub ScrollableState);

#[pyproto]
impl PyObjectProtocol for WrappedScrollableState {
    fn __str__(&self) -> PyResult<String> {
        debug_str(&self.0)
    }
}

#[pymethods]
impl WrappedScrollableState {
    #[new]
    fn new() -> Self {
        Self(Arc::new(Mutex::new(Default::default())))
    }
}

make_with_state! {
    scrollable_with_state(
        iced::Scrollable<Message>,
        iced::Scrollable<'this, Message>,
        iced::scrollable::State,
    );
}
