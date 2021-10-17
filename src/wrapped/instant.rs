use std::num::FpCategory;
use std::time::{Duration, Instant};

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::PyNumberProtocol;

use crate::common::{debug_str, EitherPy};

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedInstant>()?;
    Ok(())
}

/// Instant()
/// --
///
/// A measurement of a monotonically nondecreasing clock. Opaque and useful only with duration.
///
/// * You can add/substract a number of seconds as :class:`float` to/from an instant to get a new instant.
/// * You can add/substract a :class:`~datetime.timedelta` to/from an instant to get a new instant.
/// * You can substract two instants to get the number of seconds as :class:`float` between them: ``later - earlier = seconds``.
///
/// See also
/// --------
/// `std::time::Instant <https://doc.rust-lang.org/std/time/struct.Instant.html>`_
#[pyclass(name = "Instant", module = "pyiced")]
#[derive(Debug, Clone)]
pub(crate) struct WrappedInstant(pub Instant);

#[pymethods]
impl WrappedInstant {
    #[new]
    fn new() -> Self {
        Self(Instant::now())
    }

    fn __str__(&self) -> PyResult<String> {
        debug_str(&self.0)
    }
}

#[pyproto]
impl PyNumberProtocol for WrappedInstant {
    fn __add__(lhs: WrappedInstant, rhs: EitherPy<f64, Py<PyAny>>) -> PyResult<WrappedInstant> {
        let secs = match rhs {
            EitherPy::Left(secs) => secs,
            EitherPy::Right(duration) => Python::with_gil(|py| -> PyResult<f64> {
                duration.call_method0(py, "total_seconds")?.extract(py)
            })?,
        };
        let duration = match duration_try_from_secs_f64(secs) {
            Ok(duration) => duration,
            Err(_) => return Err(PyErr::new::<PyValueError, _>("Illegal duration")),
        };
        let result = match lhs.0.checked_add(duration) {
            Some(result) => result,
            None => return Err(PyErr::new::<PyValueError, _>("Duration too big")),
        };
        Ok(Self(result))
    }

    fn __sub__(
        lhs: WrappedInstant,
        rhs: EitherPy<f64, WrappedInstant>,
    ) -> PyResult<EitherPy<WrappedInstant, f64>> {
        match rhs {
            EitherPy::Left(rhs) => {
                let duration = match duration_try_from_secs_f64(rhs) {
                    Ok(duration) => duration,
                    Err(_) => return Err(PyErr::new::<PyValueError, _>("Illegal duration")),
                };
                let instant = match lhs.0.checked_sub(duration) {
                    Some(instant) => instant,
                    None => return Err(PyErr::new::<PyValueError, _>("Duration too big")),
                };
                Ok(EitherPy::Left(WrappedInstant(instant)))
            },

            EitherPy::Right(rhs) => {
                let duration = match lhs.0.checked_duration_since(rhs.0) {
                    Some(duration) => duration,
                    None => {
                        return Err(PyErr::new::<PyValueError, _>(
                            "Misordered 'instant's for substraction",
                        ));
                    },
                };
                Ok(EitherPy::Right(duration.as_secs_f64()))
            },
        }
    }
}

// NIGHTLY FEATURES:

fn duration_try_from_secs_f64(secs: f64) -> Result<Duration, ()> {
    const NANOS_PER_SEC: u32 = 1_000_000_000;
    const MAX_NANOS_F64: f64 = ((u64::MAX as u128 + 1) * (NANOS_PER_SEC as u128)) as f64;

    let nanos = secs * (NANOS_PER_SEC as f64);
    match nanos.classify() {
        FpCategory::Nan | FpCategory::Infinite => return Err(()),
        FpCategory::Zero | FpCategory::Subnormal => return Ok(Duration::default()),
        FpCategory::Normal => {},
    }
    if !(0.0..MAX_NANOS_F64).contains(&nanos) {
        return Err(());
    }

    let nanos = nanos as u128;
    Ok(Duration::new(
        (nanos / (NANOS_PER_SEC as u128)) as u64,
        (nanos % (NANOS_PER_SEC as u128)) as u32,
    ))
}
