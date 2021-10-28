use std::convert::TryFrom;
use std::hash::{Hash, Hasher};
use std::time::Duration;

use iced::time::every;
use iced::Subscription;
use pyo3::prelude::*;

use super::{ToSubscription, WrappedSubscription};
use crate::common::{GCProtocol, Message};
use crate::wrapped::WrappedMessage;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(make_every, m)?)?;
    Ok(())
}

#[derive(Debug, Clone)]
pub(crate) struct Every {
    duration: Duration,
    message: Message,
    hash: usize,
}

impl GCProtocol for Every {
    fn traverse(&self, visit: &pyo3::PyVisit) -> Result<(), pyo3::PyTraverseError> {
        self.message.traverse(visit)
    }
}

impl<'p> TryFrom<(Python<'p>, &'p PyAny, Py<PyAny>)> for Every {
    type Error = PyErr;

    fn try_from(value: (Python<'p>, &'p PyAny, Py<PyAny>)) -> PyResult<Self> {
        let (py, duration, message) = value;

        let seconds = match duration.extract::<f64>() {
            Ok(seconds) => seconds,
            Err(_) => duration.call_method0("total_seconds")?.extract::<f64>()?,
        };
        let duration = Duration::from_nanos((seconds * 1e9) as u64);

        let hash = py
            .import("builtins")?
            .call_method1("hash", (&message,))?
            .extract::<usize>()?;
        let message = message.extract::<WrappedMessage>(py)?.0;

        Ok(Every {
            duration,
            message,
            hash,
        })
    }
}

impl ToSubscription for Every {
    fn to_subscription(&self) -> Subscription<Message> {
        every(self.duration)
            .with(self.clone())
            .map(|(m, _)| m.message)
    }
}

impl Hash for Every {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.duration.hash(state);
        self.hash.hash(state);
    }
}

/// every($module, /, duration, message)
/// --
///
/// Returns a :class:`~pyiced.Subscription` that produces messages at a set interval.
///
/// The first :class:`~pyiced.Message` is produced after a duration, and then continues to produce more messages every duration after that.
///
/// Parameters
/// ----------
/// duration : Union[float, datetime.timedelta]
///     The interval in seconds or as a duration.
/// message : Message
///     The message to send to the :meth:`pyiced.IcedApp.update`.
///
/// Returns
/// -------
/// Subscription
///     The new subscription.
///
/// See also
/// --------
/// * `iced_futures::time::every <https://docs.rs/iced_futures/0.3.0/iced_futures/time/fn.every.html>`_
#[pyfunction(name = "every")]
fn make_every(py: Python, duration: &PyAny, message: Py<PyAny>) -> PyResult<WrappedSubscription> {
    Ok(Every::try_from((py, duration, message))?.into())
}
