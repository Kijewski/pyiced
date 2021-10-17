use std::fmt::{Debug, Write};

use pyo3::exceptions::{PyException, PyValueError};
use pyo3::{PyObjectProtocol, prelude::*};

use crate::debug_str;

macro_rules! do_wrap {
    ($($name:ident($type:ty) ),* $(,)?) => {
        pub(crate) fn init(_py: Python, m: &PyModule) -> PyResult<()> {
            $( m.add_class::<$name>()?; )*
            Ok(())
        }

        $(
            #[pyclass]
            #[derive(Debug, Clone)]
            pub(crate) struct $name(pub $type);
        )*
    };
}

macro_rules! trivial_str {
    ($($type:ty),* $(,)?) => {
        $(
            #[pyproto(module="pyiced.pyiced")]
            impl PyObjectProtocol for $type {
                fn __str__(&self) -> PyResult<String> {
                    let mut result = String::new();
                    let outcome = write!(&mut result, "{:#?}", &self.0);
                    match outcome {
                        Ok(()) => Ok(result),
                        Err(err) => Err(PyException::new_err(format!("{}", err))),
                    }
                }
            }
        )*
    };
}

do_wrap!(
    Message(crate::app::Message),
    Color(iced::Color),
    Length(iced::Length), // TODO
    Font(iced::Font), // TODO
    HorizontalAlignment(iced::HorizontalAlignment),
    VerticalAlignment(iced::VerticalAlignment),
    TooltipPosition(iced::tooltip::Position),
    Element(crate::elements::Element),
);

trivial_str!(
    Color,
    Length,
    Font,
    HorizontalAlignment,
    VerticalAlignment,
    TooltipPosition,
);

#[pymethods]
impl Color {
    #[new]
    fn new(r: f32, g: f32, b: f32, a: Option<f32>) -> Self {
        let a = a.unwrap_or(1.0);
        let v = iced::Color { r, g, b, a };
        Self(v)
    }
}

#[pymethods]
impl HorizontalAlignment {
    #[new]
    fn new(v: &str) -> PyResult<Self> {
        Ok(Self(match v {
            "<" | "l" | "left" | "Left" => iced::HorizontalAlignment::Left,
            "-" | "c" | "center" | "Center" => iced::HorizontalAlignment::Center,
            ">" | "r" | "right" | "Right" => iced::HorizontalAlignment::Right,
            _ => return Err(PyValueError::new_err(v.to_owned())),
        }))
    }
}

#[pymethods]
impl VerticalAlignment {
    #[new]
    fn new(v: &str) -> PyResult<Self> {
        Ok(Self(match v {
            "^" | "t" | "top" | "Top" => iced::VerticalAlignment::Top,
            "-" | "c" | "center" | "Center" => iced::VerticalAlignment::Center,
            "v" | "b" | "bottom" | "Bottom" => iced::VerticalAlignment::Bottom,
            _ => return Err(PyValueError::new_err(v.to_owned())),
        }))
    }
}

#[pymethods]
impl TooltipPosition {
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

#[pyproto(module="pyiced.pyiced")]
impl PyObjectProtocol for Message {
    fn __str__(&self) -> PyResult<String> {
        let value: &dyn Debug = match &self.0 {
            v @ crate::app::Message::None => v,
            crate::app::Message::Native(v) => v,
            crate::app::Message::Python(v) => v,
        };
        debug_str(value)
    }
}

#[pyproto(module="pyiced.pyiced")]
impl PyObjectProtocol for Element {
    fn __str__(&self) -> PyResult<String> {
        let value: &dyn Debug = match &self.0 {
            crate::elements::Element::NoElement(v) => v,
            crate::elements::Element::Button(v) => v,
            crate::elements::Element::Checkbox(v) => v,
            crate::elements::Element::Column(v) => v, // TODO
            crate::elements::Element::Container(v) => v, // TODO
            crate::elements::Element::Image(v) => v, // TODO
            crate::elements::Element::PickList(v) => v, // TODO
            crate::elements::Element::ProgressBar(v) => v, // TODO
            crate::elements::Element::Radio(v) => v, // TODO
            crate::elements::Element::Row(v) => v, // TODO
            crate::elements::Element::Rule(v) => v, // TODO
            crate::elements::Element::Scrollable(v) => v, // TODO
            crate::elements::Element::Slider(v) => v, // TODO
            crate::elements::Element::Space(v) => v, // TODO
            crate::elements::Element::Svg(v) => v, // TODO
            crate::elements::Element::Text(v) => v,
            crate::elements::Element::TextInput(v) => v, // TODO
            crate::elements::Element::Tooltip(v) => v,
        };
        debug_str(value)
    }
}

#[pymethods]
impl Message {
    #[new]
    fn new<'p>(data: Py<PyAny>) -> Self {
        Self(crate::app::Message::Python(data))
    }
}

#[pymethods]
impl Element {
    #[staticmethod]
    fn button<'p>(
        py: Python<'p>,
        state: Py<PyAny>,
        content: &Element,
        width: Option<&Length>,
        height: Option<&Length>,
        min_width: Option<u32>,
        min_height: Option<u32>,
        padding: Option<u16>,
        on_press: Option<&Message>,
    ) -> PyResult<&'p PyAny> {
        let value = crate::elements::Button {
            state,
            content: Box::new(content.0.clone()),
            width: width.map(|o| o.0.clone()),
            height: height.map(|o| o.0.clone()),
            min_width,
            min_height,
            padding,
            on_press: on_press.map(|o| o.0.clone()),
        };
        let value = PyCell::new(py, Self(value.into()))?;
        Ok(value)
    }

    #[staticmethod]
    fn checkbox<'p>(
        py: Python<'p>, is_checked: bool, label: String, f: Py<PyAny>, size: Option<u16>,
        width: Option<&Length>, spacing: Option<u16>, text_size: Option<u16>, font: Option<&Font>,
    ) -> PyResult<&'p PyAny> {
        let value = crate::elements::Checkbox {
            is_checked,
            label,
            f: Some(f),
            size,
            width: width.map(|o| o.0.clone()),
            spacing,
            text_size,
            font: font.map(|o| o.0.clone()),
        };
        let value = PyCell::new(py, Self(value.into()))?;
        Ok(value)
    }

    #[staticmethod]
    fn text<'p>(
        py: Python<'p>,
        label: String,
        size: Option<u16>,
        color: Option<&Color>,
        font: Option<&Font>,
        width: Option<&Length>,
        height: Option<&Length>,
        horizontal_alignment: Option<&HorizontalAlignment>,
        vertical_alignment: Option<&VerticalAlignment>,
    ) -> PyResult<&'p PyAny> {
        let value = crate::elements::Text {
            label,
            size,
            color: color.map(|o| o.0.clone()),
            font: font.map(|o| o.0.clone()),
            width: width.map(|o| o.0.clone()),
            height: height.map(|o| o.0.clone()),
            horizontal_alignment: horizontal_alignment.map(|o| o.0.clone()),
            vertical_alignment: vertical_alignment.map(|o| o.0.clone()),
        };
        let value = PyCell::new(py, Self(value.into()))?;
        Ok(value)
    }

    #[staticmethod]
    fn tooltip<'p>(
        py: Python<'p>,
        content: &Element,
        tooltip: String,
        position: &TooltipPosition,
        size: Option<u16>,
        font: Option<&Font>,
        gap: Option<u16>,
        padding: Option<u16>,
    ) -> PyResult<&'p PyAny> {
        let value = crate::elements::Tooltip {
            content: Box::new(content.0.clone()),
            tooltip,
            position: position.0.clone(),
            size,
            font: font.map(|o| o.0.clone()),
            gap,
            padding,
        };
        let value = PyCell::new(py, Self(value.into()))?;
        Ok(value)
    }
}
