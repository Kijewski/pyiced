use pyo3::{PyObjectProtocol, prelude::*};

use crate::common::{Message, debug_str};

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedMessage>()?;
    Ok(())
}

#[pyclass(name="Message", module="pyiced.pyiced")]
#[derive(Debug, Clone)]
pub(crate) struct WrappedMessage(pub Message);

#[pymethods]
impl WrappedMessage {
    #[new]
    fn new(value: Py<PyAny>) -> Self {
        WrappedMessage(Message::Python(value))
    }
}

#[pyproto]
impl PyObjectProtocol for WrappedMessage {
    fn __str__(&self) -> PyResult<String> {
        match &self.0 {
            v @ Message::None => debug_str(v),
            Message::Native(v) => debug_str(v),
            Message::Python(v) => debug_str(v),
        }
    }
}
