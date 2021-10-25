use std::fmt::{Debug, Write};

use futures_util::FutureExt;
use iced::{Command, Element, Length, Space};
use iced_native::Event;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pyo3::types::PyTuple;
use pyo3::{PyTraverseError, PyVisit};
use pyo3_asyncio::into_future_with_loop;

use crate::wrapped::WrappedMessage;

pub(crate) fn init_mod(_py: Python, _m: &PyModule) -> PyResult<()> {
    Ok(())
}

#[derive(Debug, Clone)]
pub(crate) enum Message {
    None,
    Native(Event),
    Python(Py<PyAny>),
}

pub(crate) trait ToNative {
    fn to_native(&self, py: Python<'_>) -> Element<'static, Message>;
}

pub(crate) fn debug_str(value: &dyn Debug) -> PyResult<String> {
    let mut result = String::new();
    let err = match write!(&mut result, "{:#?}", value) {
        Ok(()) => return Ok(result),
        Err(err) => err,
    };

    let mut result = String::new();
    match write!(&mut result, "{:#?}", err) {
        Ok(()) => Err(PyException::new_err(result)),
        Err(err) => {
            dbg!(err);
            Err(PyException::new_err("<EXCEPTION>"))
        },
    }
}

pub(crate) fn empty_space() -> Element<'static, Message> {
    Space::new(Length::Shrink, Length::Shrink).into()
}

pub(crate) fn to_msg_fn<T>(f: &Py<PyAny>) -> impl Fn(T) -> Message
where
    (T,): IntoPy<Py<PyTuple>>,
{
    let f = f.clone();
    move |value: T| {
        Python::with_gil(|py| match f.call1(py, (value,)) {
            Ok(value) if !value.is_none(py) => match value.extract(py) {
                Ok(WrappedMessage(message)) => message,
                Err(err) => {
                    err.print(py);
                    Message::None
                },
            },
            Ok(_) => Message::None,
            Err(err) => {
                err.print(py);
                Message::None
            },
        })
    }
}

pub(crate) fn method_into_py(py: Python, method: &PyAny) -> Option<Py<PyAny>> {
    match method.is_none() {
        true => None,
        false => Some(method.into_py(py)),
    }
}

pub(crate) fn py_to_command(
    py: Python,
    pyloop: &Py<PyAny>,
    vec: PyResult<Py<PyAny>>,
) -> Command<Message> {
    let vec = match vec {
        Ok(vec) if !vec.is_none(py) => vec,
        Ok(_) => return Command::none(),
        Err(err) => {
            err.print(py);
            return Command::none();
        },
    };
    let vec = match vec.as_ref(py).iter() {
        Ok(vec) => vec,
        Err(err) => {
            err.print(py);
            return Command::none();
        },
    };

    let vec = vec.into_iter().filter_map(|command| {
        let command = match command {
            Ok(command) => command,
            Err(err) => {
                err.print(py);
                return None;
            },
        };
        let fut = match into_future_with_loop(pyloop.as_ref(py), command) {
            Ok(fut) => fut,
            Err(err) => {
                err.print(py);
                return None;
            },
        };
        Some(Command::from(fut.map(|result| {
            Python::with_gil(|py| match result {
                Ok(msg) if !msg.is_none(py) => match msg.extract(py) {
                    Ok(WrappedMessage(msg)) => msg,
                    Err(err) => {
                        err.print(py);
                        Message::None
                    },
                },
                Ok(_) => Message::None,
                Err(err) => {
                    err.print(py);
                    Message::None
                },
            })
        })))
    });
    Command::batch(vec)
}

#[allow(unused_variables)]
pub(crate) trait GCProtocol {
    fn traverse(&self, visit: &PyVisit) -> Result<(), PyTraverseError> {
        Ok(())
    }
}
