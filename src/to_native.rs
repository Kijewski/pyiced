use std::ops::DerefMut;

use iced::{Element, Length};
use iced_native::Widget;
use iced_wgpu::Renderer;
use parking_lot::MutexGuard;
use pyo3::types::PyTuple;
use pyo3::{IntoPy, Py, PyAny, Python};

use crate::app::Message;
use crate::wrapped_states;

macro_rules! assign {
    ($input:expr, $self:ident, $name:ident $(,)?) => {
        match ($input, $self.$name) {
            (input, ::std::option::Option::Some(value)) => input.$name(value),
            (input, ::std::option::Option::None) => input,
        }
    };
    ($input:expr, $self:ident, $name:ident, $($names:ident),+ $(,)?) => {
        assign!(
            assign!($input, $self, $name),
            $self, $($names),+
        )
    };
}

fn to_msg_fn<T>(f: &Py<PyAny>) -> impl Fn(T) -> Message
where
    (T,): IntoPy<Py<PyTuple>>,
{
    let f = f.clone();
    move |value: T| Python::with_gil(|py| match f.call1(py, (value,)) {
        Ok(value) if !value.is_none(py) => match value.extract(py) {
            Ok(super::wrapped::Message(message)) => message,
            Err(err) => {
                err.print(py);
                Message::None
            }
        },
        Ok(_) => Message::None,
        Err(err) => {
            err.print(py);
            Message::None
        },
    })
}

pub(crate) trait ToNative {
    fn to_native(self, py: Python<'_>) -> iced::Element<'static, Message>;
}

impl ToNative for crate::elements::Element {
    fn to_native(self, py: Python) -> iced::Element<'static, Message> {
        match self {
            crate::elements::Element::NoElement(value) => value.to_native(py),
            crate::elements::Element::Button(value) => value.to_native(py),
            crate::elements::Element::Checkbox(value) => value.to_native(py),
            crate::elements::Element::Column(value) => value.to_native(py),
            crate::elements::Element::Container(value) => value.to_native(py),
            crate::elements::Element::Image(value) => value.to_native(py),
            crate::elements::Element::PickList(value) => value.to_native(py),
            crate::elements::Element::ProgressBar(value) => value.to_native(py),
            crate::elements::Element::Radio(value) => value.to_native(py),
            crate::elements::Element::Row(value) => value.to_native(py),
            crate::elements::Element::Rule(value) => value.to_native(py),
            crate::elements::Element::Scrollable(value) => value.to_native(py),
            crate::elements::Element::Slider(value) => value.to_native(py),
            crate::elements::Element::Space(value) => value.to_native(py),
            crate::elements::Element::Svg(value) => value.to_native(py),
            crate::elements::Element::Text(value) => value.to_native(py),
            crate::elements::Element::TextInput(value) => value.to_native(py),
            crate::elements::Element::Tooltip(value) => value.to_native(py),
        }
    }
}

struct WidgetWithGuard<W, A, G>
where
    W: Widget<Message, Renderer>,
{
    widget: W,
    guard: Option<G>,
    arc: Option<A>,
}

impl<W, A, G> Drop for WidgetWithGuard<W, A, G>
where
    W: Widget<Message, Renderer>,
{
    fn drop(&mut self) {
        self.guard.take();
        self.arc.take();
    }
}

impl<W, A, G> Widget<Message, Renderer> for WidgetWithGuard<W, A, G>
where
    W: Widget<Message, Renderer>,
{
    fn width(&self) -> Length {
        self.widget.width()
    }

    fn height(&self) -> Length {
        self.widget.height()
    }

    fn layout(
        &self,
        renderer: &Renderer,
        limits: &iced_native::layout::Limits,
    ) -> iced_native::layout::Node {
        self.widget.layout(renderer, limits)
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        defaults: &<Renderer as iced_native::renderer::Renderer>::Defaults,
        layout: iced_native::Layout<'_>,
        cursor_position: iced::Point,
        viewport: &iced::Rectangle,
    ) -> <Renderer as iced_native::renderer::Renderer>::Output {
        self.widget.draw(renderer, defaults, layout, cursor_position, viewport)
    }

    fn hash_layout(&self, state: &mut iced_native::Hasher) {
        self.widget.hash_layout(state)
    }
}

fn empty_space() -> iced::Element<'static, Message> {
    iced::Space::new(Length::Shrink, Length::Shrink).into()
}

impl ToNative for crate::elements::NoElement {
    fn to_native(self, _py: Python) -> iced::Element<'static, Message> {
        empty_space()
    }
}

impl ToNative for crate::elements::Button {
    fn to_native(self, py: Python) -> iced::Element<'static, Message> {
        let arc = Python::with_gil(|py| match self.state.extract(py) {
            Ok(wrapped_states::ButtonState(state)) => Some(state),
            Err(err) => {
                err.print(py);
                None
            }
        });
        let arc = match arc {
            Some(arc) => arc,
            None => return empty_space(),
        };
        let guard = match arc.try_lock() {
            Some(guard) => guard,
            None => return empty_space(),
        };
        let mut guard: MutexGuard<'static, iced::button::State> = unsafe { std::mem::transmute(guard) };
        let state = guard.deref_mut();
        let state: &'static mut _ = unsafe { std::mem::transmute(state) };
        
        let content = self.content.to_native(py);
        let result = iced::Button::new(state, content);
        let result = assign!(result, self, width, height, min_width, min_height, padding);
        let result = match &self.on_press {
            Some(on_press) => result.on_press(on_press.clone()),
            None => result,
        };

        let result = WidgetWithGuard {
            widget: result,
            guard: Some(guard),
            arc: Some(arc),
        };
        Element::new(result)
    }
}

impl ToNative for crate::elements::Checkbox {
    fn to_native(self, _py: Python) -> iced::Element<'static, Message> {
        let f = to_msg_fn(&self.f.unwrap());
        let el = iced::Checkbox::new(self.is_checked, &self.label, f);
        let el = assign!(el, self, size, width, spacing, text_size, font);
        el.into()
    }
}

impl ToNative for crate::elements::Column {
    fn to_native(self, py: Python) -> iced::Element<'static, Message> {
        let children = self.children.into_iter().map(|child| child.to_native(py)).collect();
        let el = iced::Column::with_children(children);
        let el = assign!(el, self, spacing, padding, width, height, max_width, max_height, align_items);
        el.into()
    }
}

impl ToNative for crate::elements::Container {
    fn to_native(self, py: Python) -> iced::Element<'static, Message> {
        let content = self.content.to_native(py);
        let el = iced::Container::new(content);
        let el = assign!(el, self, padding, width, height, max_width, max_height, align_x, align_y);
        el.into()
    }
}

impl ToNative for crate::elements::Image {
    fn to_native(self, _py: Python) -> iced::Element<'static, Message> {
        let el = iced::Image::new(self.handle.clone());
        let el = assign!(el, self, width, height);
        el.into()
    }
}

impl ToNative for crate::elements::PickList {
    fn to_native(self, _py: Python) -> iced::Element<'static, Message> {
        todo!();
        let on_select = to_msg_fn(&self.on_selected.unwrap());
        let el = iced::PickList::new(&mut self.state, &self.options[..], self.selected.clone(), on_select);
        el.into()
    }
}

impl ToNative for crate::elements::ProgressBar {
    fn to_native(self, _py: Python) -> iced::Element<'static, Message> {
        let el = iced::ProgressBar::new(self.range.clone(), self.value);
        let el = assign!(el, self, width, height);
        el.into()
    }
}

impl ToNative for crate::elements::Radio {
    fn to_native(self, _py: Python) -> iced::Element<'static, Message> {
        let f = to_msg_fn(&self.f.unwrap());
        let el = iced::Radio::new(self.value, &self.label, self.selected, f);
        let el = assign!(el, self, size, width, spacing, text_size);
        el.into()
    }
}

impl ToNative for crate::elements::Row {
    fn to_native(self, py: Python) -> iced::Element<'static, Message> {
        let children = self.children.into_iter().map(|child| child.to_native(py)).collect();
        let el = iced::Row::with_children(children);
        let el = assign!(el, self, spacing, padding, width, height, max_width, max_height, align_items);
        el.into()
    }
}

impl ToNative for crate::elements::Rule {
    fn to_native(self, _py: Python) -> iced::Element<'static, Message> {
        let el = match self {
            Self { horizontal: Some(spacing), .. } => iced::Rule::horizontal(spacing),
            Self { vertical: Some(spacing), .. } => iced::Rule::vertical(spacing),
            _ => return empty_space(),
        };
        el.into()
    }
}

impl ToNative for crate::elements::Scrollable {
    fn to_native(self, _py: Python) -> iced::Element<'static, Message> {
        todo!();
        let el = iced::Scrollable::new(&mut self.state);
        let el = assign!(
            el, self, spacing, padding, width, height, max_width, max_height, align_items,
            scrollbar_width, scrollbar_margin, scroller_width,
        );
        el.into()
    }
}

impl ToNative for crate::elements::Slider {
    fn to_native(self, _py: Python) -> iced::Element<'static, Message> {
        todo!();
        let on_change = to_msg_fn(&self.on_change.unwrap());
        let el = iced::Slider::new(&mut self.state, self.range.clone(), self.value, on_change);
        let el = assign!(el, self, width, height, step);
        let el = match &self.on_release {
            Some(on_release) => el.on_release(on_release.clone()),
            _ => el,
        };
        el.into()
    }
}

impl ToNative for crate::elements::Space {
    fn to_native(self, _py: Python) -> iced::Element<'static, Message> {
        let el = iced::Space::new(self.width, self.height);
        el.into()
    }
}

impl ToNative for crate::elements::Svg {
    fn to_native(self, _py: Python) -> iced::Element<'static, Message> {
        let el = iced::Svg::new(self.handle.clone());
        let el = assign!(el, self, width, height);
        el.into()
    }
}

impl ToNative for crate::elements::Text {
    fn to_native(self, _py: Python) -> iced::Element<'static, Message> {
        let el = iced::Text::new(&self.label);
        let el = assign!(el, self, size, color, font, width, height, horizontal_alignment, vertical_alignment);
        el.into()
    }
}

impl ToNative for crate::elements::TextInput {
    fn to_native(self, _py: Python) -> iced::Element<'static, Message> {
        todo!();
        let on_change = to_msg_fn(&self.on_change.unwrap());
        let el = iced::TextInput::new(&mut self.state, &self.placeholder, &self.value, on_change);
        let el = assign!(el, self, font, width, max_width, padding, size);
        let el = match &self.on_submit {
            Some(on_submit) => el.on_submit(on_submit.clone()),
            _ => el,
        };
        let el = match self.password {
            true => el.password(),
            false => el,
        };
        el.into()
    }
}

impl ToNative for crate::elements::Tooltip {
    fn to_native(self, py: Python) -> iced::Element<'static, Message> {
        let content = self.content.to_native(py);
        let el = iced::Tooltip::new(content, &self.tooltip, self.position);
        let el = assign!(el, self, size, font, gap, padding);
        el.into()
    }
}
