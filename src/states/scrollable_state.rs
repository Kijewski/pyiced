use std::num::FpCategory;
use std::sync::Arc;

use iced::scrollable::State;
use parking_lot::RwLock;
use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;

use crate::common::debug_str;
use crate::make_with_state;
use crate::wrapped::WrappedRectangle;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedScrollableState>()?;
    Ok(())
}

pub(crate) type ScrollableState = Arc<RwLock<State>>;

/// ScrollableState()
/// --
///
/// The state of a :func:`~pyiced.scrollable()`.
///
/// Warning
/// -------
/// If the state is currently in use, calling its methods will fail.
#[pyclass(name = "ScrollableState", module = "pyiced")]
#[derive(Debug, Default, Clone)]
pub(crate) struct WrappedScrollableState(pub ScrollableState);

#[pymethods]
impl WrappedScrollableState {
    #[new]
    fn new() -> Self {
        Self::default()
    }

    /// scroll($self, /, delta_y, bounds, content_bounds)
    /// --
    ///
    /// Apply a scrolling offset to the current ScrollableState, given the bounds of the Scrollable and its contents.
    ///
    /// Arguments
    /// ---------
    /// delta_y : float
    ///     TODO
    /// bounds : Rectangle
    ///     TODO
    /// content_bounds : Rectangle
    ///     TODO
    fn scroll(
        &mut self,
        delta_y: f32,
        bounds: &WrappedRectangle,
        content_bounds: &WrappedRectangle,
    ) -> PyResult<()> {
        match delta_y.classify() {
            FpCategory::Nan => {
                return Err(PyErr::new::<PyValueError, _>("delta_y must not be NaN"));
            },
            FpCategory::Zero | FpCategory::Subnormal => return Ok(()),
            FpCategory::Infinite | FpCategory::Normal => {},
        }
        match self.0.try_write() {
            Some(mut guard) => {
                guard.scroll(delta_y, bounds.0, content_bounds.0);
                Ok(())
            },
            None => Err(PyErr::new::<PyRuntimeError, _>("State is in use")),
        }
    }

    /// scroll_to($self, /, percentage, bounds, content_bounds)
    /// --
    ///
    /// Moves the scroll position to a relative amount, given the bounds of the Scrollable and its contents.
    ///
    /// 0.0 represents scrollbar at the top, while 1.0 represents scrollbar at the bottom.
    ///
    /// Arguments
    /// ---------
    /// percentage : float
    ///     TODO
    /// bounds : Rectangle
    ///     TODO
    /// content_bounds : Rectangle
    ///     TODO
    fn scroll_to(
        &mut self,
        percentage: f32,
        bounds: &WrappedRectangle,
        content_bounds: &WrappedRectangle,
    ) -> PyResult<()> {
        let percentage = match percentage.classify() {
            FpCategory::Nan => {
                return Err(PyErr::new::<PyValueError, _>("delta_y must not be NaN"));
            },
            FpCategory::Zero | FpCategory::Subnormal => 0.0f32,
            FpCategory::Infinite | FpCategory::Normal => match percentage {
                c if c < 0.0f32 => 0.0f32,
                c if c > 1.0f32 => 1.0f32,
                c => c,
            },
        };
        match self.0.try_write() {
            Some(mut guard) => {
                guard.scroll_to(percentage, bounds.0, content_bounds.0);
                Ok(())
            },
            None => Err(PyErr::new::<PyRuntimeError, _>("State is in use")),
        }
    }

    /// offset($self, /, bounds, content_bounds)
    /// --
    ///
    /// The current scrolling offset of the ScrollableState, given the bounds of the Scrollable and its contents.
    ///
    /// Arguments
    /// ---------
    /// bounds : Rectangle
    ///     TODO
    /// content_bounds : Rectangle
    ///     TODO
    ///
    /// Returns
    /// -------
    /// int
    ///     The scrolling offset.
    fn offset(
        &self,
        bounds: &WrappedRectangle,
        content_bounds: &WrappedRectangle,
    ) -> PyResult<u32> {
        match self.0.try_read() {
            Some(guard) => Ok(guard.offset(bounds.0, content_bounds.0)),
            None => Err(PyErr::new::<PyRuntimeError, _>("State is in use")),
        }
    }

    /// is_scroller_grabbed($self)
    /// --
    ///
    /// Returns whether the scroller is currently grabbed or not.
    ///
    /// Returns
    /// -------
    /// bool
    ///     Yes or no
    fn is_scroller_grabbed(&self) -> PyResult<bool> {
        match self.0.try_read() {
            Some(guard) => Ok(guard.is_scroller_grabbed()),
            None => Err(PyErr::new::<PyRuntimeError, _>("State is in use")),
        }
    }

    /// is_scroll_box_touched($self)
    /// --
    ///
    /// Returns whether the scroll box is currently touched or not.
    ///
    /// Returns
    /// -------
    /// bool
    ///     Yes or no
    fn is_scroll_box_touched(&self) -> PyResult<bool> {
        match self.0.try_read() {
            Some(guard) => Ok(guard.is_scroll_box_touched()),
            None => Err(PyErr::new::<PyRuntimeError, _>("State is in use")),
        }
    }

    fn __str__(&self) -> PyResult<String> {
        debug_str(&self.0)
    }
}

make_with_state! {
    scrollable_with_state(
        iced::Scrollable<Message>,
        iced::Scrollable<'this, Message>,
        iced::scrollable::State,
    );
}
