use std::sync::Arc;

use parking_lot::Mutex;
use pyo3::{PyObjectProtocol, prelude::*};

use crate::common::debug_str;
use crate::make_with_state;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedScrollableState>()?;
    Ok(())
}

pub(crate) type ScrollableState = Arc<Mutex<iced::scrollable::State>>;

#[pyclass(name="ScrollableState", module="pyiced.pyiced")]
#[derive(Debug, Default)]
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
