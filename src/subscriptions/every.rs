use std::any::TypeId;
use std::convert::TryFrom;
use std::hash::{Hash, Hasher};
use std::time::Duration;

use iced::time::every;
use iced::Subscription;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyTuple;

use super::{ToSubscription, WrappedSubscription};
use crate::app::Interop;
use crate::common::{GCProtocol, Message};
use crate::wrapped::WrappedInstant;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(make_every, m)?)?;
    Ok(())
}

#[derive(Debug, Clone)]
pub(crate) struct Every {
    duration: Duration,
    token: Py<PyAny>,
    hash: usize,
}

impl GCProtocol for Every {
    fn traverse(&self, visit: &pyo3::PyVisit) -> Result<(), pyo3::PyTraverseError> {
        visit.call(&self.token)
    }
}

impl<'p> TryFrom<(Python<'p>, &'p PyAny, Py<PyAny>)> for Every {
    type Error = PyErr;

    fn try_from(value: (Python<'p>, &'p PyAny, Py<PyAny>)) -> PyResult<Self> {
        let (py, duration, token) = value;

        let seconds = match duration.extract::<f64>() {
            Ok(seconds) => seconds,
            Err(_) => duration.call_method0("total_seconds")?.extract::<f64>()?,
        };
        if !seconds.is_normal() {
            // either nan, infinite, subnormal, or zero
            return Err(PyErr::new::<PyValueError, _>("Duration must be positive"));
        }
        let duration = Duration::from_nanos((seconds * 1e9) as u64);
        if duration.as_micros() < 100 {
            // To prevent crashes in iced_graphics::window::Compositor::draw():
            // "Next frame: Outdated" while resizing window.
            return Err(PyErr::new::<PyValueError, _>(
                "Duration must be at least 100 µs.",
            ));
        }

        let hash = py
            .import("builtins")?
            .call_method1("hash", (&token,))?
            .extract::<usize>()?;

        Ok(Every {
            duration,
            token,
            hash,
        })
    }
}

impl ToSubscription for Every {
    fn to_subscription(&self, _interop: &Interop) -> Subscription<Message> {
        every(self.duration).with(self.clone()).map(|(m, instant)| {
            Python::with_gil(|py| {
                Message::Python(
                    PyTuple::new(py, &[m.token, WrappedInstant(instant).into_py(py)]).into(),
                )
            })
        })
    }
}

impl Hash for Every {
    fn hash<H: Hasher>(&self, state: &mut H) {
        struct Marker;
        TypeId::of::<Marker>().hash(state);
        self.duration.hash(state);
        self.hash.hash(state);
    }
}

/// every($module, /, duration, token)
/// --
///
/// Returns a :class:`~pyiced.Subscription` that produces messages at a set interval.
///
/// The first :class:`~pyiced.Message` is produced after a duration, and then continues to produce more messages every duration after that.
///
/// Parameters
/// ----------
/// duration : Union[float, datetime.timedelta]
///     The interval in seconds or as a duration. Must be at least 100 µs!
/// token : Object
///     The first item of the message tuple to send to the :meth:`pyiced.IcedApp.update`.
///
/// Returns
/// -------
/// Subscription
///     The new subscription.
///
///     Every "duration" a message ``Message((token, instant))`` is sent to :meth:`pyiced.IcedApp.update`.
///
///     .. seealso::
///         :class:`~pyiced.Instant`.
///
/// See also
/// --------
/// `iced_futures::time::every <https://docs.rs/iced_futures/0.3.0/iced_futures/time/fn.every.html>`_
#[pyfunction(name = "every")]
fn make_every(py: Python, duration: &PyAny, token: Py<PyAny>) -> PyResult<WrappedSubscription> {
    Ok(Every::try_from((py, duration, token))?.into())
}
