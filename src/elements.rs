use std::ops::RangeInclusive;

use pyo3::{Py, PyAny};

use crate::app::Message;

macro_rules! elements {
    ($($name:ident),+ $(,)?) => {
        #[derive(Debug, Clone)]
        pub(crate) enum Element {
            $( $name($name) ),+
        }

        $(
            impl From<$name> for Element {
                fn from(value: $name) -> Element {
                    Element::$name(value)
                }
            }
        )+
    };
}

elements!(
    NoElement,
    Button,
    // TODO: Canvas
    Checkbox,
    Column,
    Container,
    Image,
    // TODO: PaneGrid
    PickList,
    ProgressBar,
    Radio,
    Row,
    Rule,
    Scrollable,
    Slider,
    Space,
    Svg,
    Text,
    TextInput,
    Tooltip,
);

#[derive(Default, Debug, Clone, Copy)]
pub(crate) struct NoElement;

type NonOptPy<T> = Option<T>;

#[derive(Debug, Clone)]
pub(crate) struct Button {
    pub state: Py<PyAny>,
    pub content: Box<Element>,
    pub width: Option<iced::Length>,
    pub height: Option<iced::Length>,
    pub min_width: Option<u32>,
    pub min_height: Option<u32>,
    pub padding: Option<u16>,
    pub on_press: Option<Message>,
    // style: TODO,
}

#[derive(Debug, Clone)]
pub(crate) struct Checkbox {
    pub is_checked: bool,
    pub label: String,
    pub f: NonOptPy<Py<PyAny>>, // fn f(checked: bool) -> crate::Message
    pub size: Option<u16>,
    pub width: Option<iced::Length>,
    pub spacing: Option<u16>,
    pub text_size: Option<u16>,
    pub font: Option<iced::Font>,
    // style: TODO,
}

#[derive(Debug, Clone)]
pub(crate) struct Column {
    pub children: Vec<Element>,
    pub spacing: Option<u16>,
    pub padding: Option<u16>,
    pub width: Option<iced::Length>,
    pub height: Option<iced::Length>,
    pub max_width: Option<u32>,
    pub max_height: Option<u32>,
    pub align_items: Option<iced::Align>,
}

#[derive(Debug, Clone)]
pub(crate) struct Container {
    pub content: Box<Element>,
    pub padding: Option<u16>,
    pub width: Option<iced::Length>,
    pub height: Option<iced::Length>,
    pub max_width: Option<u32>,
    pub max_height: Option<u32>,
    pub align_x: Option<iced::Align>,
    pub align_y: Option<iced::Align>,
    // style: TODO,
}

#[derive(Debug, Clone)]
pub(crate) struct Image {
    pub handle: iced::image::Handle,
    pub width: Option<iced::Length>,
    pub height: Option<iced::Length>,
}

#[derive(Debug, Clone)]
pub(crate) struct PickList {
    pub state: iced::pick_list::State<String>,
    pub options: Vec<String>,
    pub selected: Option<String>,
    pub on_selected: NonOptPy<Py<PyAny>>, // fn on_selected(value: String) -> crate::Message
}

#[derive(Debug, Clone)]
pub(crate) struct ProgressBar {
    pub range: RangeInclusive<f32>,
    pub value: f32,
    pub width: Option<iced::Length>,
    pub height: Option<iced::Length>,
    // style: TODO,
}

#[derive(Debug, Clone)]
pub(crate) struct Radio {
    pub value: i64,
    pub label: String,
    pub selected: Option<i64>,
    pub f: NonOptPy<Py<PyAny>>, // fn f(value: i64) -> crate::Message
    pub size: Option<u16>,
    pub width: Option<iced::Length>,
    pub spacing: Option<u16>,
    pub text_size: Option<u16>,
    // style: TODO,
}

#[derive(Debug, Clone)]
pub(crate) struct Row {
    pub children: Vec<Element>,
    pub spacing: Option<u16>,
    pub padding: Option<u16>,
    pub width: Option<iced::Length>,
    pub height: Option<iced::Length>,
    pub max_width: Option<u32>,
    pub max_height: Option<u32>,
    pub align_items: Option<iced::Align>,
}

#[derive(Debug, Clone)]
pub(crate) struct Rule {
    pub horizontal: Option<u16>,
    pub vertical: Option<u16>,
    // style: TODO,
}

#[derive(Debug, Clone)]
pub(crate) struct Scrollable {
    pub state: iced::scrollable::State,
    pub spacing: Option<u16>,
    pub padding: Option<u16>,
    pub width: Option<iced::Length>,
    pub height: Option<iced::Length>,
    pub max_width: Option<u32>,
    pub max_height: Option<u32>,
    pub align_items: Option<iced::Align>,
    pub scrollbar_width: Option<u16>,
    pub scrollbar_margin: Option<u16>,
    pub scroller_width: Option<u16>,
    // style: TODO,
}

#[derive(Debug, Clone)]
pub(crate) struct Slider {
    pub state: iced::slider::State,
    pub range: RangeInclusive<f32>,
    pub value: f32,
    pub on_change: NonOptPy<Py<PyAny>>, // fn f(value: Float) -> crate::Message
    pub on_release: Option<Message>,
    pub width: Option<iced::Length>,
    pub height: Option<u16>,
    pub step: Option<f32>,
    // style: TODO,
}

#[derive(Debug, Clone)]
pub(crate) struct Space {
    pub width: iced::Length,
    pub height: iced::Length,
}

#[derive(Debug, Clone)]
pub(crate) struct Svg {
    pub handle: iced::svg::Handle,
    pub width: Option<iced::Length>,
    pub height: Option<iced::Length>,
}

#[derive(Debug, Clone)]
pub(crate) struct Text {
    pub label: String,
    pub size: Option<u16>,
    pub color: Option<iced::Color>,
    pub font: Option<iced::Font>,
    pub width: Option<iced::Length>,
    pub height: Option<iced::Length>,
    pub horizontal_alignment: Option<iced::HorizontalAlignment>,
    pub vertical_alignment: Option<iced::VerticalAlignment>,
}

#[derive(Debug, Clone)]
pub(crate) struct TextInput {
    pub state: iced::text_input::State,
    pub placeholder: String,
    pub value: String,
    pub on_change: NonOptPy<Py<PyAny>>, // fn f(value: String) -> crate::Message
    pub font: Option<iced::Font>,
    pub width: Option<iced::Length>,
    pub max_width: Option<u32>,
    pub padding: Option<u16>,
    pub size: Option<u16>,
    pub on_submit: Option<Message>,
    pub password: bool,
    // style: TODO,
}

#[derive(Debug, Clone)]
pub(crate) struct Tooltip {
    pub content: Box<Element>,
    pub tooltip: String,
    pub position: iced::tooltip::Position,
    pub size: Option<u16>,
    pub font: Option<iced::Font>,
    pub gap: Option<u16>,
    pub padding: Option<u16>,
    // style: TODO,
}
