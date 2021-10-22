use pyo3::prelude::*;
use pyo3::{PyGCProtocol, PyObjectProtocol, PyTraverseError, PyVisit};

use crate::common::{debug_str, Message};

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedMessage>()?;
    Ok(())
}

#[pyclass(name = "Message", module = "pyiced", gc)]
#[derive(Debug, Clone)]
pub(crate) struct WrappedMessage(pub Message);

#[pyproto]
impl PyGCProtocol for WrappedMessage {
    fn __traverse__(&self, visit: PyVisit) -> Result<(), PyTraverseError> {
        if let Message::Python(obj) = &self.0 {
            visit.call(obj)?;
        }
        Ok(())
    }

    fn __clear__(&mut self) {
        *self = WrappedMessage(Message::None);
    }
}

#[pymethods]
impl WrappedMessage {
    #[new]
    fn new(value: Py<PyAny>) -> Self {
        WrappedMessage(Message::Python(value))
    }

    #[getter]
    fn r#type(&self) -> &'static str {
        match &self.0 {
            Message::None => "none",
            Message::Native(_) => "native",
            Message::Python(_) => "python",
        }
    }

    #[getter]
    fn python(&self, py: Python) -> Py<PyAny> {
        match &self.0 {
            Message::Python(value) => value.into_py(py),
            _ => ().into_py(py),
        }
    }

    #[getter]
    fn native(&self, py: Python) -> Py<PyAny> {
        let v = match get_native(self) {
            Ok(v) => v,
            Err(_) => return ().into_py(py),
        };
        match v {
            iced_native::Event::Mouse(_) => "mouse",
            iced_native::Event::Window(_) => "window",
            iced_native::Event::Touch(_) => "touch",
            iced_native::Event::Keyboard(_) => "keyboard",
        }
        .into_py(py)
    }

    // keyboard

    #[getter]
    fn keyboard(&self, py: Python) -> Py<PyAny> {
        let v = match get_keyboard(self) {
            Ok(v) => v,
            Err(_) => return ().into_py(py),
        };
        match v {
            iced::keyboard::Event::KeyPressed { .. } => "keypressed",
            iced::keyboard::Event::KeyReleased { .. } => "keyreleased",
            iced::keyboard::Event::CharacterReceived(_) => "characterreceived",
            iced::keyboard::Event::ModifiersChanged(_) => "modifierschanged",
        }
        .into_py(py)
    }

    #[getter]
    fn key_code(&self, py: Python) -> Py<PyAny> {
        let v = match get_keyboard(self) {
            Ok(v) => v,
            Err(_) => return ().into_py(py),
        };
        match v {
            iced::keyboard::Event::KeyPressed { key_code, .. }
            | iced::keyboard::Event::KeyReleased { key_code, .. } => {
                format!("{:?}", key_code).into_py(py)
            },
            _ => ().into_py(py),
        }
    }

    #[getter]
    fn shift(&self, py: Python) -> Py<PyAny> {
        let v = match get_keyboard(self) {
            Ok(v) => v,
            Err(_) => return ().into_py(py),
        };
        match v {
            iced::keyboard::Event::KeyPressed { modifiers, .. }
            | iced::keyboard::Event::KeyReleased { modifiers, .. } => modifiers.shift.into_py(py),
            _ => ().into_py(py),
        }
    }

    #[getter]
    fn alt(&self, py: Python) -> Py<PyAny> {
        let v = match get_keyboard(self) {
            Ok(v) => v,
            Err(_) => return ().into_py(py),
        };
        match v {
            iced::keyboard::Event::KeyPressed { modifiers, .. }
            | iced::keyboard::Event::KeyReleased { modifiers, .. } => modifiers.alt.into_py(py),
            _ => ().into_py(py),
        }
    }

    #[getter]
    fn logo(&self, py: Python) -> Py<PyAny> {
        let v = match get_keyboard(self) {
            Ok(v) => v,
            Err(_) => return ().into_py(py),
        };
        match v {
            iced::keyboard::Event::KeyPressed { modifiers, .. }
            | iced::keyboard::Event::KeyReleased { modifiers, .. }
            | iced::keyboard::Event::ModifiersChanged(modifiers) => modifiers.logo.into_py(py),
            _ => ().into_py(py),
        }
    }

    #[getter]
    fn control(&self, py: Python) -> Py<PyAny> {
        let v = match get_keyboard(self) {
            Ok(v) => v,
            Err(_) => return ().into_py(py),
        };
        match v {
            iced::keyboard::Event::KeyPressed { modifiers, .. }
            | iced::keyboard::Event::KeyReleased { modifiers, .. }
            | iced::keyboard::Event::ModifiersChanged(modifiers) => modifiers.control.into_py(py),
            _ => ().into_py(py),
        }
    }

    #[getter]
    fn characterreceived(&self, py: Python) -> Py<PyAny> {
        let v = match get_keyboard(self) {
            Ok(v) => v,
            Err(_) => return ().into_py(py),
        };
        match v {
            iced::keyboard::Event::CharacterReceived(c) => format!("{}", c).into_py(py),
            _ => ().into_py(py),
        }
    }

    // mouse

    #[getter]
    fn mouse(&self, py: Python) -> Py<PyAny> {
        let v = match get_mouse(self) {
            Ok(v) => v,
            Err(_) => return ().into_py(py),
        };
        match v {
            iced::mouse::Event::CursorEntered => "cursorentered",
            iced::mouse::Event::CursorLeft => "cursorleft",
            iced::mouse::Event::CursorMoved { .. } => "cursormoved",
            iced::mouse::Event::ButtonPressed(_) => "buttonpressed",
            iced::mouse::Event::ButtonReleased(_) => "buttonreleased",
            iced::mouse::Event::WheelScrolled { .. } => "wheelscrolled",
        }
        .into_py(py)
    }

    #[getter]
    fn cursormoved(&self, py: Python) -> Py<PyAny> {
        let v = match get_mouse(self) {
            Ok(v) => v,
            Err(_) => return ().into_py(py),
        };
        match v {
            iced::mouse::Event::CursorMoved { position } => (position.x, position.y).into_py(py),
            _ => ().into_py(py),
        }
    }

    #[getter]
    fn button(&self, py: Python) -> Py<PyAny> {
        let v = match get_mouse(self) {
            Ok(v) => v,
            Err(_) => return ().into_py(py),
        };
        match v {
            iced::mouse::Event::ButtonPressed(b) | iced::mouse::Event::ButtonReleased(b) => match b
            {
                iced::mouse::Button::Left => "left".into_py(py),
                iced::mouse::Button::Right => "right".into_py(py),
                iced::mouse::Button::Middle => "middle".into_py(py),
                iced::mouse::Button::Other(i) => (*i as u32).into_py(py),
            },
            _ => ().into_py(py),
        }
    }

    #[getter]
    fn wheelscrolled(&self, py: Python) -> Py<PyAny> {
        let v = match get_mouse(self) {
            Ok(v) => v,
            Err(_) => return ().into_py(py),
        };
        match v {
            iced::mouse::Event::WheelScrolled { delta } => match delta {
                iced::mouse::ScrollDelta::Lines { .. } => "lines",
                iced::mouse::ScrollDelta::Pixels { .. } => "pixels",
            }
            .into_py(py),
            _ => ().into_py(py),
        }
    }

    #[getter]
    fn amount(&self, py: Python) -> Py<PyAny> {
        let v = match get_mouse(self) {
            Ok(v) => v,
            Err(_) => return ().into_py(py),
        };
        match v {
            iced::mouse::Event::WheelScrolled { delta } => match delta {
                iced::mouse::ScrollDelta::Lines { x, y }
                | iced::mouse::ScrollDelta::Pixels { x, y } => (*x, *y).into_py(py),
            },
            _ => ().into_py(py),
        }
    }

    // touch

    #[getter]
    fn touch(&self, py: Python) -> Py<PyAny> {
        let v = match get_touch(self) {
            Ok(v) => v,
            Err(_) => return ().into_py(py),
        };
        match v {
            iced_native::touch::Event::FingerPressed { .. } => "fingerpressed",
            iced_native::touch::Event::FingerMoved { .. } => "fingermoved",
            iced_native::touch::Event::FingerLifted { .. } => "fingerlifted",
            iced_native::touch::Event::FingerLost { .. } => "fingerlost",
        }
        .into_py(py)
    }

    #[getter]
    fn finger(&self, py: Python) -> Py<PyAny> {
        let v = match get_touch(self) {
            Ok(v) => v,
            Err(_) => return ().into_py(py),
        };
        match v {
            iced_native::touch::Event::FingerPressed { id, .. }
            | iced_native::touch::Event::FingerMoved { id, .. }
            | iced_native::touch::Event::FingerLifted { id, .. }
            | iced_native::touch::Event::FingerLost { id, .. } => id.0.into_py(py),
        }
    }

    #[getter]
    fn position(&self, py: Python) -> Py<PyAny> {
        let v = match get_touch(self) {
            Ok(v) => v,
            Err(_) => return ().into_py(py),
        };
        match v {
            iced_native::touch::Event::FingerPressed { position, .. }
            | iced_native::touch::Event::FingerMoved { position, .. }
            | iced_native::touch::Event::FingerLifted { position, .. }
            | iced_native::touch::Event::FingerLost { position, .. } => {
                (position.x, position.y).into_py(py)
            },
        }
    }

    // window

    #[getter]
    fn window(&self, py: Python) -> Py<PyAny> {
        let v = match get_window(self) {
            Ok(v) => v,
            Err(_) => return ().into_py(py),
        };
        match v {
            iced_native::window::Event::Resized { .. } => "resized",
            iced_native::window::Event::CloseRequested => "closerequested",
            iced_native::window::Event::Focused => "focused",
            iced_native::window::Event::Unfocused => "unfocused",
            iced_native::window::Event::FileHovered(_) => "filehovered",
            iced_native::window::Event::FileDropped(_) => "filedropped",
            iced_native::window::Event::FilesHoveredLeft => "fileshoveredleft",
        }
        .into_py(py)
    }

    #[getter]
    fn resized(&self, py: Python) -> Py<PyAny> {
        let v = match get_window(self) {
            Ok(v) => v,
            Err(_) => return ().into_py(py),
        };
        match v {
            iced_native::window::Event::Resized { width, height } => (*width, *height).into_py(py),
            _ => ().into_py(py),
        }
    }

    #[getter]
    fn file(&self, py: Python) -> Py<PyAny> {
        let v = match get_window(self) {
            Ok(v) => v,
            Err(_) => return ().into_py(py),
        };
        match v {
            iced_native::window::Event::FileHovered(path)
            | iced_native::window::Event::FileDropped(path) => path.into_py(py),
            _ => ().into_py(py),
        }
    }
}

fn get_native(v: &WrappedMessage) -> Result<&iced_native::Event, ()> {
    match &v.0 {
        Message::Native(ev) => Ok(ev),
        _ => Err(()),
    }
}

fn get_keyboard(v: &WrappedMessage) -> Result<&iced_native::keyboard::Event, ()> {
    match get_native(v)? {
        iced_native::Event::Keyboard(ev) => Ok(ev),
        _ => Err(()),
    }
}

fn get_mouse(v: &WrappedMessage) -> Result<&iced_native::mouse::Event, ()> {
    match get_native(v)? {
        iced_native::Event::Mouse(ev) => Ok(ev),
        _ => Err(()),
    }
}

fn get_touch(v: &WrappedMessage) -> Result<&iced_native::touch::Event, ()> {
    match get_native(v)? {
        iced_native::Event::Touch(ev) => Ok(ev),
        _ => Err(()),
    }
}

fn get_window(v: &WrappedMessage) -> Result<&iced_native::window::Event, ()> {
    match get_native(v)? {
        iced_native::Event::Window(ev) => Ok(ev),
        _ => Err(()),
    }
}

#[pyproto]
impl PyObjectProtocol for WrappedMessage {
    fn __str__(&self) -> PyResult<String> {
        match &self.0 {
            v @ Message::None => debug_str(v),
            Message::Native(v) => debug_str(v),
            Message::Python(v) => debug_str(v),
        }
    }
}
