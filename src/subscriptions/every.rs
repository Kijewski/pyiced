use std::any::TypeId;
use std::convert::TryFrom;
use std::hash::{Hash, Hasher};
use std::time::Duration;

use iced::time::every;
use iced::Subscription;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyDelta, PyDeltaAccess, PyTuple};

use super::{ToSubscription, WrappedSubscription};
use crate::app::Interop;
use crate::common::{EitherPy, GCProtocol, Message};
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

impl<'p> TryFrom<(Python<'p>, EitherPy<&PyDelta, f64>, Py<PyAny>)> for Every {
    type Error = PyErr;

    fn try_from(value: (Python<'p>, EitherPy<&PyDelta, f64>, Py<PyAny>)) -> PyResult<Self> {
        let (py, duration, token) = value;

        let duration = match duration {
            EitherPy::Left(delta) => {
                let days = delta.get_days();
                if days < 0 {
                    return Err(PyErr::new::<PyValueError, _>("Duration must be positive"));
                }

                let days = days as u64;
                let secs = delta.get_seconds() as u64;
                let micros = delta.get_microseconds() as u32;

                let secs = match days
                    .checked_mul(24 * 60 * 60)
                    .and_then(|s| secs.checked_add(s))
                {
                    Some(s) => s,
                    None => return Err(PyErr::new::<PyValueError, _>("Duration too big")),
                };
                let nanos = micros * 1000;
                Duration::new(secs, nanos)
            },
            EitherPy::Right(seconds) => {
                if !seconds.is_normal() || seconds < 0.0 {
                    // either nan, infinite, subnormal, or zero
                    return Err(PyErr::new::<PyValueError, _>("Duration must be positive"));
                }
                let nanos = seconds * 1e9;
                if !nanos.is_normal() || nanos > u64::MAX as _ {
                    return Err(PyErr::new::<PyValueError, _>("Duration too big"));
                };
                Duration::from_nanos(nanos as u64)
            },
        };

        if duration.as_micros() < 100 {
            // To prevent crashes in iced_graphics::window::Compositor::draw():
            // "Next frame: Outdated" while resizing window.
            return Err(PyErr::new::<PyValueError, _>(
                "Duration must be at least 100 µs.",
            ));
        }

        let hash = token.as_ref(py).hash()? as _;

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
///     Every "duration" a message ``(token, instant)`` is sent to :meth:`pyiced.IcedApp.update`.
///
///     .. seealso::
///         :class:`~pyiced.Instant`.
///
/// See also
/// --------
/// `iced_futures::time::every <https://docs.rs/iced_futures/0.3.0/iced_futures/time/fn.every.html>`_
#[pyfunction(name = "every")]
fn make_every(
    py: Python,
    duration: EitherPy<&PyDelta, f64>,
    token: Py<PyAny>,
) -> PyResult<WrappedSubscription> {
    Ok(Every::try_from((py, duration, token))?.into())
}
