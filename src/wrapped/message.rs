use std::path::PathBuf;

use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::{PyObjectProtocol, prelude::*, types::{PyTuple, PyString, PyLong}};

use crate::common::{Message, debug_str};

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedMessage>()?;
    Ok(())
}

#[pyclass(name="Message", module="pyiced.pyiced")]
#[derive(Debug, Clone)]
pub(crate) struct WrappedMessage(pub Message);

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
    fn none(&self) -> PyResult<()> {
        match &self.0 {
            Message::None => Ok(()),
            _ => Err(PyValueError::new_err("")),
        }
    }

    #[getter]
    fn python(&self) -> PyResult<&Py<PyAny>> {
        match &self.0 {
            Message::Python(value) => Ok(value),
            _ => Err(PyValueError::new_err("")),
        }
    }

    #[getter]
    fn native(&self) -> PyResult<&'static str> {
        match get_native(self)? {
            iced_native::Event::Mouse(_) => return Ok("mouse"),
            iced_native::Event::Window(_) => return Ok("window"),
            iced_native::Event::Touch(_) => return Ok("touch"),
            iced_native::Event::Keyboard(_) => return Ok("keyboard"),
        }
    }

    // keyboard

    #[getter]
    fn keyboard(&self) -> PyResult<&'static str> {
        match get_keyboard(self)? {
            iced::keyboard::Event::KeyPressed { ..} => Ok("keypressed"),
            iced::keyboard::Event::KeyReleased { .. } => Ok("keyreleased"),
            iced::keyboard::Event::CharacterReceived(_) => Ok("characterreceived"),
            iced::keyboard::Event::ModifiersChanged(_) => Ok("modifierschanged"),
        }
    }

    #[getter]
    fn key_code(&self) -> PyResult<String> {
        match get_keyboard(self)? {
            iced::keyboard::Event::KeyPressed { key_code, .. }
            | iced::keyboard::Event::KeyReleased { key_code, .. } => Ok(format!("{:?}", key_code)),
            _ => Err(PyValueError::new_err("")),
        }
    }

    #[getter]
    fn shift(&self) -> PyResult<bool> {
        match get_keyboard(self)? {
            iced::keyboard::Event::KeyPressed { modifiers, .. }
            | iced::keyboard::Event::KeyReleased { modifiers, .. } => Ok(modifiers.shift),
            _ => Err(PyValueError::new_err("")),
        }
    }

    #[getter]
    fn alt(&self) -> PyResult<bool> {
        match get_keyboard(self)? {
            iced::keyboard::Event::KeyPressed { modifiers, .. }
            | iced::keyboard::Event::KeyReleased { modifiers, .. } => Ok(modifiers.alt),
            _ => Err(PyValueError::new_err("")),
        }
    }

    #[getter]
    fn logo(&self) -> PyResult<bool> {
        match get_keyboard(self)? {
            iced::keyboard::Event::KeyPressed { modifiers, .. }
            | iced::keyboard::Event::KeyReleased { modifiers, .. }
            | iced::keyboard::Event::ModifiersChanged(modifiers) => Ok(modifiers.logo),
            _ => Err(PyValueError::new_err("")),
        }
    }

    #[getter]
    fn control(&self) -> PyResult<bool> {
        match get_keyboard(self)? {
            iced::keyboard::Event::KeyPressed { modifiers, .. }
            | iced::keyboard::Event::KeyReleased { modifiers, .. }
            | iced::keyboard::Event::ModifiersChanged(modifiers) => Ok(modifiers.control),
            _ => Err(PyValueError::new_err("")),
        }
    }

    #[getter]
    fn characterreceived(&self) -> PyResult<String> {
        match get_keyboard(self)? {
            iced::keyboard::Event::CharacterReceived(c) => Ok(format!("{}", c)),
            _ => Err(PyValueError::new_err("")),
        }
    }

    // mouse

    #[getter]
    fn mouse(&self) -> PyResult<&'static str> {
        match get_mouse(self)? {
            iced::mouse::Event::CursorEntered => Ok("cursorentered"),
            iced::mouse::Event::CursorLeft => Ok("cursorleft"),
            iced::mouse::Event::CursorMoved { .. } => Ok("cursormoved"),
            iced::mouse::Event::ButtonPressed(_) => Ok("buttonpressed"),
            iced::mouse::Event::ButtonReleased(_) => Ok("buttonreleased"),
            iced::mouse::Event::WheelScrolled { .. } => Ok("wheelscrolled"),
        }
    }

    #[getter]
    fn cursormoved(&self) -> PyResult<(f32, f32)> {
        match get_mouse(self)? {
            iced::mouse::Event::CursorMoved { position } => Ok((position.x, position.y)),
            _ => Err(PyValueError::new_err("")),
        }
    }

    #[getter]
    fn button(&self, py: Python) -> PyResult<Py<PyAny>> {
        match get_mouse(self)? {
            iced::mouse::Event::ButtonPressed(b)
            | iced::mouse::Event::ButtonReleased(b) => match b {
                iced::mouse::Button::Left => Ok("left".into_py(py)),
                iced::mouse::Button::Right => Ok("right".into_py(py)),
                iced::mouse::Button::Middle => Ok("middle".into_py(py)),
                iced::mouse::Button::Other(i) => Ok((*i as u32).into_py(py)),
            },
            _ => Err(PyValueError::new_err("")),
        }
    }

    #[getter]
    fn wheelscrolled(&self) -> PyResult<&'static str> {
        match get_mouse(self)? {
            iced::mouse::Event::WheelScrolled { delta } => match delta {
                iced::mouse::ScrollDelta::Lines { .. } => Ok("lines"),
                iced::mouse::ScrollDelta::Pixels { .. } => Ok("pixels"),
            },
            _ => Err(PyValueError::new_err("")),
        }
    }

    #[getter]
    fn amount(&self) -> PyResult<(f32, f32)> {
        match get_mouse(self)? {
            iced::mouse::Event::WheelScrolled { delta } => match delta {
                iced::mouse::ScrollDelta::Lines { x, y }
                | iced::mouse::ScrollDelta::Pixels { x, y } => Ok((*x, *y)),
            },
            _ => Err(PyValueError::new_err("")),
        }
    }

    // touch: TODO

    #[getter]
    fn touch(&self) -> PyResult<&'static str> {
        match get_touch(self)? {
            iced_native::touch::Event::FingerPressed { .. } => Ok("fingerpressed"),
            iced_native::touch::Event::FingerMoved { .. } => Ok("fingermoved"),
            iced_native::touch::Event::FingerLifted { .. } => Ok("fingerlifted"),
            iced_native::touch::Event::FingerLost { .. } => Ok("fingerlost"),
        }
    }

    #[getter]
    fn finger(&self) -> PyResult<u64> {
        match get_touch(self)? {
            iced_native::touch::Event::FingerPressed { id, .. }
            | iced_native::touch::Event::FingerMoved { id, .. }
            | iced_native::touch::Event::FingerLifted { id, .. }
            | iced_native::touch::Event::FingerLost { id, .. } => Ok(id.0),
        }
    }

    #[getter]
    fn position(&self) -> PyResult<(f32, f32)> {
        match get_touch(self)? {
            iced_native::touch::Event::FingerPressed { position, .. }
            | iced_native::touch::Event::FingerMoved { position, .. }
            | iced_native::touch::Event::FingerLifted { position, .. }
            | iced_native::touch::Event::FingerLost { position, .. } => {
                Ok((position.x, position.y))
            }
        }
    }

    // window: TODO

    #[getter]
    fn window(&self) -> PyResult<&'static str> {
        match get_window(self)? {
            iced_native::window::Event::Resized { .. } => Ok("resized"),
            iced_native::window::Event::CloseRequested => Ok("closerequested"),
            iced_native::window::Event::Focused => Ok("focused"),
            iced_native::window::Event::Unfocused => Ok("unfocused"),
            iced_native::window::Event::FileHovered(_) => Ok("filehovered"),
            iced_native::window::Event::FileDropped(_) => Ok("filedropped"),
            iced_native::window::Event::FilesHoveredLeft => Ok("fileshoveredleft"),
        }
    }

    #[getter]
    fn resized(&self) -> PyResult<(u32, u32)> {
        match get_window(self)? {
            iced_native::window::Event::Resized { width, height } => Ok((*width, *height)),
            _ => Err(PyValueError::new_err("")),
        }
    }

    #[getter]
    fn file(&self) -> PyResult<&PathBuf> {
        match get_window(self)? {
            iced_native::window::Event::FileHovered(path)
            | iced_native::window::Event::FileDropped(path) => Ok(path),
            _ => Err(PyValueError::new_err("")),
        }
    }
}

fn get_native(v: &WrappedMessage) -> PyResult<&iced_native::Event> {
    match &v.0 {
        Message::Native(ev) => Ok(ev),
        _ => Err(PyValueError::new_err("")),
    }
}

fn get_keyboard(v: &WrappedMessage) -> PyResult<&iced_native::keyboard::Event> {
    match get_native(v)? {
        iced_native::Event::Keyboard(ev) => Ok(ev),
        _ => Err(PyValueError::new_err("")),
    }
}

fn get_mouse(v: &WrappedMessage) -> PyResult<&iced_native::mouse::Event> {
    match get_native(v)? {
        iced_native::Event::Mouse(ev) => Ok(ev),
        _ => Err(PyValueError::new_err("")),
    }
}

fn get_touch(v: &WrappedMessage) -> PyResult<&iced_native::touch::Event> {
    match get_native(v)? {
        iced_native::Event::Touch(ev) => Ok(ev),
        _ => Err(PyValueError::new_err("")),
    }
}

fn get_window(v: &WrappedMessage) -> PyResult<&iced_native::window::Event> {
    match get_native(v)? {
        iced_native::Event::Window(ev) => Ok(ev),
        _ => Err(PyValueError::new_err("")),
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
