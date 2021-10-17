use pyo3::exceptions::PyValueError;
use pyo3::{PyObjectProtocol, prelude::*};

use crate::common::debug_str;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedTooltipPosition>()?;
    Ok(())
}

#[pyclass(name="TooltipPosition", module="pyiced.pyiced")]
#[derive(Debug, Clone)]
pub(crate) struct WrappedTooltipPosition(pub iced::tooltip::Position);

#[pymethods]
impl WrappedTooltipPosition {
    #[new]
    fn new(v: &str) -> PyResult<Self> {
        Ok(Self(match v {
            "-" | "f" | "follow_cursor" | "FollowCursor" => iced::tooltip::Position::FollowCursor,
            "^" | "t" | "top" | "Top" => iced::tooltip::Position::Top,
            "v" | "b" | "bottom" | "Bottom" => iced::tooltip::Position::Bottom,
            "<" | "l" | "left" | "Left" => iced::tooltip::Position::Left,
            ">" | "r" | "right" | "Right" => iced::tooltip::Position::Right,
            _ => return Err(PyValueError::new_err(v.to_owned())),
        }))
    }
}

#[pyproto]
impl PyObjectProtocol for WrappedTooltipPosition {
    fn __str__(&self) -> PyResult<String> {
        debug_str(&self.0)
    }
}
