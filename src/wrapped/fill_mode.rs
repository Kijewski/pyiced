use std::borrow::Cow;
use std::num::FpCategory;

use iced::widget::rule::FillMode;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use crate::common::debug_str;
use crate::format_to_cow;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedFillMode>()?;
    Ok(())
}

/// The fill mode of a rule.
///
/// See also
/// --------
/// `iced::widget::rule::FillMode <https://docs.rs/iced/0.3.0/iced/widget/rule/enum.FillMode.html>`_
#[pyclass(name = "FillMode", module = "pyiced")]
#[derive(Debug, Clone)]
pub(crate) struct WrappedFillMode(pub FillMode);

#[pymethods]
impl WrappedFillMode {
    /// Fill the whole length of the container.
    #[classattr]
    #[allow(non_snake_case)]
    fn FULL() -> Self {
        Self(FillMode::Full)
    }

    /// percent(percentage)
    /// --
    ///
    /// Fill a percent of the length of the container. The rule will be centered in that container.
    ///
    /// Arguments
    /// ---------
    /// percentage : float
    ///     The range is [0.0, 100.0]. The value gets clamped in this range automatically.
    #[staticmethod]
    fn percent(percentage: f32) -> PyResult<Self> {
        let percentage = match percentage.classify() {
            FpCategory::Nan => {
                return Err(PyErr::new::<PyValueError, _>(
                    "The percentage must be finite",
                ));
            },
            FpCategory::Zero | FpCategory::Subnormal => 0.0f32,
            FpCategory::Normal | FpCategory::Infinite => match percentage {
                c if c >= 100.0f32 => 100.0f32,
                c if c <= 0.0f32 => 0.0f32,
                c => c,
            },
        };
        Ok(Self(FillMode::Percent(percentage)))
    }

    /// padded(i)
    /// --
    ///
    /// Uniform offset from each end.
    ///
    /// Arguments
    /// ---------
    /// i : int
    ///     Length units.
    #[staticmethod]
    fn padded(i: u16) -> Self {
        Self(FillMode::Padded(i))
    }

    /// asymmetric_padding(first_pad, second_pad)
    /// --
    ///
    /// Different offset on each end of the rule.
    ///
    /// Arguments
    /// ---------
    /// first_pad : int
    ///     top or left, length units
    /// second_pad : int
    ///     the other direction, length units
    #[staticmethod]
    fn asymmetric_padding(first_pad: u16, second_pad: u16) -> Self {
        Self(FillMode::AsymmetricPadding(first_pad, second_pad))
    }

    fn __str__(&self) -> PyResult<String> {
        debug_str(&self.0)
    }

    fn __repr__(&self) -> PyResult<Cow<'static, str>> {
        match self.0 {
            FillMode::Full => Ok(Cow::Borrowed("FillMode.FULL")),
            FillMode::Percent(v) => format_to_cow!("FillMode.percent({:?})", v),
            FillMode::Padded(i) => format_to_cow!("FillMode.padded({:?})", i),
            FillMode::AsymmetricPadding(a, b) => {
                format_to_cow!("FillMode.asymmetric_padding({:?}, {:?})", a, b)
            },
        }
    }
}
