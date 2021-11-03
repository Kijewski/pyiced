use std::any::TypeId;
use std::hash::{Hash, Hasher};

use futures_util::stream::{unfold, BoxStream};
use iced::Subscription;
use iced_native::subscription::Recipe;
use pyo3::exceptions::{PyStopAsyncIteration, PyTypeError};
use pyo3::prelude::*;
use tokio::sync::oneshot::channel;

use super::ToSubscription;
use crate::app::{Interop, Task};
use crate::common::{GCProtocol, Message};
use crate::subscriptions::WrappedSubscription;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(make_stream, m)?)?;
    Ok(())
}

#[derive(Debug, Clone)]
pub(crate) struct Stream {
    asend: Py<PyAny>,
    id: usize,
}

#[derive(Debug, Clone)]
struct AppStream {
    stream: Stream,
    put_task: Py<PyAny>,
}

impl GCProtocol for Stream {
    fn traverse(&self, visit: &pyo3::PyVisit) -> Result<(), pyo3::PyTraverseError> {
        visit.call(&self.asend)
    }
}

impl Hash for Stream {
    fn hash<H: Hasher>(&self, state: &mut H) {
        struct Marker;
        TypeId::of::<Marker>().hash(state);
        self.id.hash(state);
    }
}

impl Hash for AppStream {
    fn hash<H: Hasher>(&self, state: &mut H) {
        struct Marker;
        TypeId::of::<Marker>().hash(state);
        self.stream.hash(state);
    }
}

impl<'p> TryFrom<(Python<'p>, Py<PyAny>)> for Stream {
    type Error = PyErr;

    fn try_from(value: (Python<'p>, Py<PyAny>)) -> PyResult<Self> {
        let (py, async_generator) = value;

        let asend = async_generator.getattr(py, "asend")?;
        if asend.is_none(py) {
            // invalid
            return Err(PyErr::new::<PyTypeError, _>("AsyncGenerator.asend is None"));
        }

        let id = py
            .import("builtins")?
            .call_method1("id", (&async_generator,))?
            .extract::<usize>()?;

        Ok(Stream { asend, id })
    }
}

impl ToSubscription for Stream {
    fn to_subscription(&self, interop: &Interop) -> Subscription<Message> {
        Subscription::from_recipe(AppStream {
            stream: self.clone(),
            put_task: interop.put_task.clone(),
        })
    }
}

impl<H, E> Recipe<H, E> for AppStream
where
    H: Hasher,
{
    type Output = Message;

    fn hash(&self, state: &mut H) {
        Hash::hash(&self, state)
    }

    fn stream(self: Box<Self>, _input: BoxStream<E>) -> BoxStream<Self::Output> {
        Box::pin(unfold(
            (self.stream.asend.clone(), self.put_task.clone()),
            move |(asend, put_task)| async move {
                let receiver = Python::with_gil(|py| {
                    // get asend(None)
                    let task = match asend.call1(py, ((),)) {
                        Ok(task) => task,
                        Err(err) => {
                            err.print(py);
                            return None;
                        },
                    };
                    if task.is_none(py) {
                        // invalid
                        PyErr::new::<PyTypeError, _>("AsyncGenerator.asend(None) returned None")
                            .print(py);
                        return None;
                    }

                    // turn into task
                    let (sender, receiver) = channel();
                    let task = Task {
                        task,
                        result: ().into_py(py),
                        done: Some(sender),
                    };
                    if let Err(err) = put_task.call1(py, (task,)) {
                        err.print(py);
                        return None;
                    }

                    Some(receiver)
                })?;

                // await result
                let outcome = match receiver.await {
                    Ok(outcome) => outcome,
                    Err(_) => return None, // the sender was GC collected in the meantime
                };

                // unwrap result
                let result = Python::with_gil(|py| {
                    let tuple = match outcome.extract::<(Option<Py<PyAny>>, Py<PyAny>)>(py) {
                        Ok(tuple) => tuple,
                        Err(err) => {
                            err.print(py);
                            return None;
                        },
                    };
                    let result = match tuple {
                        (Some(err), _) => {
                            let err = PyErr::from_instance(err.as_ref(py));
                            if !err.is_instance::<PyStopAsyncIteration>(py) {
                                err.print(py);
                            }
                            return None;
                        },
                        (None, result) => result,
                    };
                    Some(result)
                })?;

                Some((Message::Python(result), (asend, put_task)))
            },
        ))
    }
}

/// TODO
#[pyfunction(name = "stream")]
fn make_stream(py: Python, async_generator: Py<PyAny>) -> PyResult<WrappedSubscription> {
    Ok(Stream::try_from((py, async_generator))?.into())
}
