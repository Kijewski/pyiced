use pyo3::{prelude::*, wrap_pyfunction};

use crate::assign;
use crate::common::{Message, ToNative};
use crate::widgets::WrappedWidgetBuilder;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(make_svg, m)?)?;
    Ok(())
}

#[derive(Debug, Clone)]
pub(crate) struct SvgBuilder {
    pub handle: iced::svg::Handle,
    pub width: Option<iced::Length>,
    pub height: Option<iced::Length>,
}

#[pyfunction(name="svg")]
fn make_svg<'p>(
) -> WrappedWidgetBuilder {
    todo!()
}

impl ToNative for SvgBuilder {
    fn to_native(&self, _py: Python) -> iced::Element<'static, Message> {
        let el = iced::Svg::new(self.handle.clone());
        let el = assign!(el, self, width, height);
        el.into()
    }
}
