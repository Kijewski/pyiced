use iced::svg::Handle;
use iced::{Element, Length, Svg};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::assign;
use crate::common::{GCProtocol, Message, ToNative};
use crate::widgets::WrappedWidgetBuilder;
use crate::wrapped::{WrappedLength, WrappedSvgHandle};

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(make_svg, m)?)?;
    Ok(())
}

#[derive(Debug, Clone)]
pub(crate) struct SvgBuilder {
    pub handle: Handle,
    pub width: Option<Length>,
    pub height: Option<Length>,
}

impl GCProtocol for SvgBuilder {}

#[pyfunction(name = "svg")]
/// svg($module, /, handle, *, width=None, height=None)
/// --
///
/// A vector graphics image.
///
/// An SVG image resizes smoothly without losing any quality.
///
/// SVG images can have a considerable rendering cost when resized, specially when they are complex.
///
/// Parameters
/// ----------
/// handle : SvgHandle
///     The handle of the image.
/// width : Optional[Length]
///     The width of the image.
/// heigth : Optional[Length]
///     The height of the image.
///
/// Returns
/// -------
/// Element
///     The newly created SVG image.
///
/// Example
/// -------
/// .. image:: ../examples/widgets/svg.png
///    :width: 688
///    :height: 405
///    :align: center
///    :alt:
///
/// .. literalinclude :: ../examples/widgets/svg.py
///    :language: python
///
/// See also
/// --------
/// `iced_native::widget::svg::Svg <https://docs.rs/iced_native/0.4.0/iced_native/widget/svg/struct.Svg.html>`_
fn make_svg(
    handle: &WrappedSvgHandle,
    width: Option<&WrappedLength>,
    height: Option<&WrappedLength>,
) -> WrappedWidgetBuilder {
    let el = SvgBuilder {
        handle: handle.0.clone(),
        width: width.map(|o| o.0),
        height: height.map(|o| o.0),
    };
    el.into()
}

impl ToNative for SvgBuilder {
    fn to_native(&self, _py: Python) -> Element<'static, Message> {
        let el = Svg::new(self.handle.clone());
        let el = assign!(el, self, width, height);
        el.into()
    }
}
