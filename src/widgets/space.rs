use iced::{Element, Length, Space};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::common::{GCProtocol, Message, ToNative};
use crate::widgets::WrappedWidgetBuilder;
use crate::wrapped::WrappedLength;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(make_space, m)?)?;
    Ok(())
}

#[derive(Debug, Clone)]
pub(crate) struct SpaceBuilder {
    pub width: Length,
    pub height: Length,
}

impl GCProtocol for SpaceBuilder {}

#[pyfunction(name = "space")]
/// space($module, *, width=None, height=None)
/// --
///
/// An amount of empty space.
///
/// It can be useful if you want to fill some space with nothing.
///
/// Parameters
/// ----------
/// width : Optional[Length]
///     Creates an amount of horizontal space.
/// height : Optional[Length]
///     Creates an amount of vertical space.
///
/// Returns
/// -------
/// Element
///     The newly created .
///
/// See also
/// --------
/// `iced_native::widget::space::Space <https://docs.rs/iced_native/0.4.0/iced_native/widget/space/struct.Space.html>`_
fn make_space(
    width: Option<&WrappedLength>,
    height: Option<&WrappedLength>,
) -> WrappedWidgetBuilder {
    let width = match width {
        Some(width) => width.0,
        None => Length::Shrink,
    };
    let height = match height {
        Some(height) => height.0,
        None => Length::Shrink,
    };
    SpaceBuilder { width, height }.into()
}

impl ToNative for SpaceBuilder {
    fn to_native(&self, _py: Python) -> Element<'static, Message> {
        let el = Space::new(self.width, self.height);
        el.into()
    }
}
