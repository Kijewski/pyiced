use std::ops::DerefMut;

use ouroboros::self_referencing;
use parking_lot::lock_api::ArcMutexGuard;
use parking_lot::RawMutex;
use pyo3::{prelude::*, wrap_pyfunction};

use crate::assign;
use crate::common::{Message, NonOptional, ToNative, empty_space};
use crate::states::{WrappedButtonState, ButtonState};
use crate::widgets::{WidgetBuilder, WrappedWidgetBuilder};
use crate::wrapped::{WrappedMessage, WrappedLength};

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(make_button, m)?)?;
    Ok(())
}

#[derive(Debug, Clone)]
pub(crate) struct ButtonBuilder {
    pub state: NonOptional<ButtonState>,
    pub content: Box<WidgetBuilder>,
    pub width: Option<iced::Length>,
    pub height: Option<iced::Length>,
    pub min_width: Option<u32>,
    pub min_height: Option<u32>,
    pub padding: Option<u16>,
    pub on_press: Option<Message>,
    // style: TODO,
}

#[pyfunction(name="button")]
fn make_button<'p>(
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
        width: width.map(|o| o.0.clone()),
        height: height.map(|o| o.0.clone()),
        min_width,
        min_height,
        padding,
        on_press: on_press.map(|o| o.0.clone()),
    }.into()
}

#[self_referencing]
struct ButtonAndState {
    guard: ArcMutexGuard<RawMutex, iced::button::State>,
    #[borrows(mut guard)]
    #[not_covariant]
    widget: iced::Button<'this, Message>,
}

impl iced_native::Widget<Message, iced_wgpu::Renderer> for ButtonAndState {
    fn width(&self) -> iced::Length {
        self.with_widget(|w| w.width())
    }

    fn height(&self) -> iced::Length {
        self.with_widget(|w| w.height())
    }

    fn layout(
        &self,
        renderer: &iced_wgpu::Renderer,
        limits: &iced_native::layout::Limits,
    ) -> iced_native::layout::Node {
        self.with_widget(|w| w.layout(renderer, limits))
    }

    fn draw(
        &self,
        renderer: &mut iced_wgpu::Renderer,
        defaults: &<iced_wgpu::Renderer as iced_native::Renderer>::Defaults,
        layout: iced_native::Layout<'_>,
        cursor_position: iced::Point,
        viewport: &iced::Rectangle,
    ) -> <iced_wgpu::Renderer as iced_native::Renderer>::Output {
        self.with_widget(|w| w.draw(renderer, defaults, layout, cursor_position, viewport))
    }

    fn hash_layout(&self, state: &mut iced_native::Hasher) {
        self.with_widget(|w| w.hash_layout(state))
    }

    fn on_event(
        &mut self,
        event: iced_native::Event,
        layout: iced_native::Layout<'_>,
        cursor_position: iced::Point,
        renderer: &iced_wgpu::Renderer,
        clipboard: &mut dyn iced_native::Clipboard,
        messages: &mut Vec<Message>,
    ) -> iced_native::event::Status {
        self.with_widget_mut(|w| w.on_event(event, layout, cursor_position, renderer, clipboard, messages))
    }

    // fn overlay(
    //     &mut self,
    //     layout: iced_native::Layout<'_>,
    // ) -> Option<iced_native::overlay::Element<'_, Message, iced_wgpu::Renderer>> {
    //     self.with_widget_mut(|w| w.overlay(layout))
    // }
}

impl ToNative for ButtonBuilder {
    fn to_native(&self, py: Python) -> iced::Element<'static, Message> {
        let guard = self.state.as_ref().and_then(|arc| arc.try_lock_arc());
        let guard = match guard {
            Some(guard) => guard,
            None => return empty_space(),
        };
        iced::Element::new(ButtonAndStateBuilder {
            guard,
            widget_builder: |guard| {
                let content = self.content.to_native(py);
                let el = iced::Button::new(guard.deref_mut(), content);
                let el = assign!(el, self, width, height, min_width, min_height, padding);
                let el = match &self.on_press {
                    Some(on_press) => el.on_press(on_press.clone()),
                    None => el,
                };
                el
            },
        }.build())
    }
}
