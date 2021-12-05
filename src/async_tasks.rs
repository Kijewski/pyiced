use std::pin::Pin;

use futures_util::Future;
use iced::Command;
use pyo3::exceptions::{PyRuntimeError, PyTypeError};
use pyo3::prelude::*;
use tokio::sync::oneshot::{channel, Sender};

use crate::common::{debug_err, Message};
use crate::wrapped::MessageOrDatum;

pub(crate) fn init_mod(_py: Python, _m: &PyModule) -> PyResult<()> {
    Ok(())
}

pub(crate) enum MessageOrFuture {
    Message(Message),
    Future(Pin<Box<dyn Future<Output = Py<PyAny>> + Send>>),
    None,
}

#[pyclass]
pub(crate) struct Task {
    #[pyo3(get)]
    pub task: Py<PyAny>,

    #[pyo3(set)]
    pub result: Py<PyAny>,

    pub done: Option<Sender<Py<PyAny>>>,
}

#[pymethods]
impl Task {
    fn __call__(&mut self) -> PyResult<()> {
        let sender = match self.done.take() {
            Some(done) => done,
            None => return Err(PyErr::new::<PyRuntimeError, _>("Already sent")),
        };
        if sender.send(self.result.clone()).is_err() {
            // the receiver was GC collected in the meantime
        };
        Ok(())
    }
}

fn message_or_future(
    py: Python,
    result: PyResult<&PyAny>,
    put_task: &Py<PyAny>,
) -> MessageOrFuture {
    let task_or_message = match result {
        Ok(task) => task,
        Err(err) => {
            err.print(py);
            return MessageOrFuture::None;
        },
    };
    if task_or_message.is_none() {
        return MessageOrFuture::None;
    }
    match task_or_message.hasattr("__await__") {
        Ok(hasattr) => {
            if !hasattr {
                return MessageOrFuture::Message(Message::Python(task_or_message.into_py(py)));
            }
        },
        Err(err) => {
            err.print(py);
            return MessageOrFuture::None;
        },
    }

    let (sender, receiver) = channel();
    let task = Task {
        task: task_or_message.into_py(py),
        result: py.None(),
        done: Some(sender),
    };
    if let Err(err) = put_task.call1(py, (task,)) {
        err.print(py);
        return MessageOrFuture::None;
    }

    MessageOrFuture::Future(Box::pin(async move {
        let outcome = receiver.await;
        let err = match outcome {
            Ok(outcome) => return outcome,
            Err(err) => debug_err::<PyRuntimeError, _>(err),
        };
        Python::with_gil(|py| {
            err.print(py);
            py.None()
        })
    }))
}

fn future_to_command(future: Pin<Box<dyn Future<Output = Py<PyAny>> + Send>>) -> Command<Message> {
    Command::perform(future, |result| {
        Python::with_gil(|py| {
            let result = result.as_ref(py);
            let result = match result.extract::<(Option<&PyAny>, _)>() {
                Ok(result) => result,
                Err(err) => {
                    debug_err::<PyTypeError, _>(err).print(py);
                    return Message::None;
                },
            };
            match result {
                (Some(err), _) => {
                    PyErr::from_instance(err).print(py);
                    Message::None
                },
                (None, MessageOrDatum(result)) => result,
            }
        })
    })
}

pub(crate) fn vec_to_command(
    py: Python,
    vec: Py<PyAny>,
    put_task: &Py<PyAny>,
) -> PyResult<Command<Message>> {
    let vec = match vec.as_ref(py) {
        vec if vec.is_none() => return Ok(Command::none()),
        vec => vec,
    };

    let mut commands = Vec::new();
    for datum in vec.iter()?.take(64) {
        let datum = message_or_future(py, datum, put_task);
        let command = match datum {
            MessageOrFuture::Message(msg) => Command::from(async move { msg }),
            MessageOrFuture::Future(future) => future_to_command(future),
            MessageOrFuture::None => continue,
        };
        commands.push(command)
    }
    Ok(Command::batch(commands))
}
