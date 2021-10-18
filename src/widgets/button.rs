use iced::{Button, Element, Length};
use pyo3::{prelude::*, wrap_pyfunction};

use crate::assign;
use crate::common::{GCProtocol, Message, NonOptional, ToNative};
use crate::states::{WrappedButtonState, ButtonState, button_with_state};
use crate::widgets::{WidgetBuilder, WrappedWidgetBuilder};
use crate::wrapped::{WrappedMessage, WrappedLength};

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(make_button, m)?)?;
    Ok(())
}

#[derive(Debug, Clone, Default)]
pub(crate) struct ButtonBuilder {
    pub state: NonOptional<ButtonState>,
    pub content: Box<WidgetBuilder>,
    pub width: Option<Length>,
    pub height: Option<Length>,
    pub min_width: Option<u32>,
    pub min_height: Option<u32>,
    pub padding: Option<u16>,
    pub on_press: Option<Message>,
    // style: TODO,
}

impl GCProtocol for ButtonBuilder {
    fn traverse(&self, visit: &pyo3::PyVisit) -> Result<(), pyo3::PyTraverseError> {
        self.content.traverse(visit)
    }
}

#[pyfunction(name="button")]
fn make_button(
    state: &WrappedButtonState,
    content: &WrappedWidgetBuilder,
    width: Option<&WrappedLength>,
    height: Option<&WrappedLength>,
    min_width: Option<u32>,
    min_height: Option<u32>,
    padding: Option<u16>,
    on_press: Option<&WrappedMessage>,
) -> WrappedWidgetBuilder {
    ButtonBuilder {
        state: Some(state.0.clone()),
        content: Box::new(content.0.clone()),
        width: width.map(|o| o.0),
        height: height.map(|o| o.0),
        min_width,
        min_height,
        padding,
        on_press: on_press.map(|o| o.0.clone()),
    }.into()
}

impl ToNative for ButtonBuilder {
    fn to_native(&self, py: Python) -> Element<'static, Message> {
        button_with_state(self.state.as_ref(), |state| {
            let content = self.content.to_native(py);
            let el = Button::new(state, content);
            let el = assign!(el, self, width, height, min_width, min_height, padding);
            let el = match &self.on_press {
                Some(on_press) => el.on_press(on_press.clone()),
                None => el,
            };
            Ok(el)
        })
    }
}
