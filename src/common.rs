use std::fmt::{Debug, Write};

use iced::{Element, Length, Space};
use iced_native::Event;
use pyo3::exceptions::{PyException, PyTypeError};
use pyo3::prelude::*;
use pyo3::types::PyTuple;
use pyo3::{PyTraverseError, PyVisit};

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

#[allow(unused_variables)]
pub(crate) trait GCProtocol {
    fn traverse(&self, visit: &PyVisit) -> Result<(), PyTraverseError> {
        Ok(())
    }
}

impl GCProtocol for Message {
    fn traverse(&self, visit: &PyVisit) -> Result<(), PyTraverseError> {
        match self {
            Message::Python(python) => visit.call(python),
            _ => Ok(()),
        }
    }
}

pub(crate) enum EitherPy<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> ToPyObject for EitherPy<L, R>
where
    L: ToPyObject,
    R: ToPyObject,
{
    fn to_object(&self, py: Python) -> Py<PyAny> {
        match self {
            EitherPy::Left(left) => left.to_object(py),
            EitherPy::Right(right) => right.to_object(py),
        }
    }
}

impl<'source, L, R> FromPyObject<'source> for EitherPy<L, R>
where
    L: FromPyObject<'source>,
    R: FromPyObject<'source>,
{
    fn extract(src: &'source PyAny) -> PyResult<Self> {
        if let Ok(result) = L::extract(src) {
            Ok(Self::Left(result))
        } else if let Ok(result) = R::extract(src) {
            Ok(Self::Right(result))
        } else {
            match src.get_type().name() {
                Ok(name) => Err(PyErr::new::<PyTypeError, _>(format!(
                    "Unexpected type: {:?}",
                    name
                ))),
                Err(_) => Err(PyErr::new::<PyTypeError, _>("Unexpected type")),
            }
        }
    }
}

impl<T, L, R> IntoPy<T> for EitherPy<L, R>
where
    Self: Sized,
    L: IntoPy<T>,
    R: IntoPy<T>,
{
    fn into_py(self, py: Python<'_>) -> T {
        match self {
            EitherPy::Left(left) => left.into_py(py),
            EitherPy::Right(right) => right.into_py(py),
        }
    }
}
