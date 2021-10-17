use std::borrow::Cow;
use std::fmt::{Debug, Write};
use std::num::FpCategory;

use iced::{Element, Length, Space};
use iced_native::Event;
use pyo3::exceptions::{PyException, PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyTuple;
use pyo3::{PyTraverseError, PyTypeInfo, PyVisit};

use crate::format_to_string_ignore;
use crate::wrapped::MessageOrDatum;

pub(crate) fn init_mod(_py: Python, _m: &PyModule) -> PyResult<()> {
    Ok(())
}

#[derive(Clone)]
pub(crate) enum Message {
    None,
    Native(Event),
    Python(Py<PyAny>),
}

impl Debug for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let obj = match self {
            Self::None => return write!(f, "None"),
            Self::Native(arg) => return f.debug_tuple("Native").field(arg).finish(),
            Self::Python(obj) => obj,
        };

        Python::with_gil(|py| -> Result<(), std::fmt::Error> {
            match obj.as_ref(py).repr() {
                Ok(obj) => write!(f, "Python({})", obj.to_string_lossy()),
                Err(_) => write!(f, "Python({})", obj),
            }
        })
    }
}

impl Default for Message {
    fn default() -> Self {
        Self::None
    }
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
        Python::with_gil(
            |py| match f.call1(py, (value,)).and_then(|res| res.extract(py)) {
                Ok(MessageOrDatum(message)) => message,
                Err(err) => {
                    err.print(py);
                    Message::None
                },
            },
        )
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
            let s = match src.get_type().name() {
                Ok(name) => format_to_string_ignore!("Unexpected type: {:?}", name),
                Err(_) => Cow::Borrowed("Unexpected type"),
            };
            Err(PyErr::new::<PyTypeError, _>(s))
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

pub(crate) fn debug_err<T: PyTypeInfo, E: Debug>(err: E) -> PyErr {
    let mut s = String::new();
    let s = match write!(s, "{:?}", err) {
        Ok(_) => Cow::Owned(s),
        Err(_) => Cow::Borrowed("Could not even debug display the error messages"),
    };
    PyErr::new::<T, _>(s)
}

pub(crate) fn validate_f32(value: f32) -> PyResult<f32> {
    match value.classify() {
        FpCategory::Normal => Ok(value),
        FpCategory::Zero | FpCategory::Subnormal => Ok(0.0f32),
        FpCategory::Nan | FpCategory::Infinite => {
            Err(PyErr::new::<PyValueError, _>("float value must be finite"))
        },
    }
}

pub(crate) fn validate_f32_nonneg(value: f32) -> PyResult<f32> {
    let value = validate_f32(value)?;
    if value < 0.0f32 {
        return Err(PyErr::new::<PyValueError, _>("float value must be >= 0"));
    }
    Ok(value)
}

pub(crate) fn f32_nonneg(value: f32) -> PyResult<f32> {
    match value.classify() {
        FpCategory::Nan => Err(PyErr::new::<PyValueError, _>("float value must not be NaN")),
        FpCategory::Zero | FpCategory::Subnormal => Ok(0.0f32),
        FpCategory::Normal | FpCategory::Infinite => match value {
            c if c < 0.0f32 => Err(PyErr::new::<PyValueError, _>("float value must be >= 0")),
            c => Ok(c),
        },
    }
}
