use std::fmt::{Debug, Write};

use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use pyo3::{PyGCProtocol, PyObjectProtocol, PyTraverseError, PyVisit};

use crate::common::Message;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedMessage>()?;
    Ok(())
}

/// TODO
#[pyclass(name = "Message", module = "pyiced", gc)]
#[derive(Clone)]
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
    /// TODO
    #[new]
    fn new(python: Py<PyAny>) -> Self {
        WrappedMessage(Message::Python(python))
    }

    /// TODO
    #[getter]
    fn r#type(&self) -> &'static str {
        match &self.0 {
            Message::None => "none",
            Message::Native(_) => "native",
            Message::Python(_) => "python",
        }
    }

    /// TODO
    #[getter]
    fn python(&self, py: Python) -> Py<PyAny> {
        match &self.0 {
            Message::Python(value) => value.into_py(py),
            _ => ().into_py(py),
        }
    }

    /// TODO
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

    /// TODO
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

    /// TODO
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

    /// TODO
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

    /// TODO
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

    /// TODO
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

    /// TODO
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

    /// TODO
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

    /// TODO
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

    /// TODO
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

    /// TODO
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

    /// TODO
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

    /// TODO
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

    /// TODO
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

    /// A unique identifier representing a finger on a touch interaction.
    ///
    /// Returns
    /// -------
    /// int
    ///     Identifier of the finger.
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

    /// A 2D point for the touch interaction.
    ///
    /// Returns
    /// -------
    /// Tuple[float, float]
    ///     A 2D point
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

    /// TODO
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

    /// TODO
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

    /// TODO
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

    #[classattr]
    #[allow(non_snake_case)]
    fn __match_args__() -> (&'static str,) {
        // Structural Pattern Matching: https://www.python.org/dev/peps/pep-0634/
        ("python",)
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
        Python::with_gil(|py| {
            let dorepr = DoRepr { msg: self, py };
            let mut result = String::new();
            match write!(result, "{:#?}", dorepr) {
                Ok(()) => Ok(result),
                Err(_) => Err(PyErr::new::<PyRuntimeError, _>(
                    "Could not stringify Message.",
                )),
            }
        })
    }

    fn __repr__(&self) -> PyResult<String> {
        Python::with_gil(|py| {
            let dorepr = DoRepr { msg: self, py };
            let mut result = String::new();
            match write!(result, "{:?}", dorepr) {
                Ok(()) => Ok(result),
                Err(_) => Err(PyErr::new::<PyRuntimeError, _>(
                    "Could not stringify Message.",
                )),
            }
        })
    }
}

struct DoRepr<'m, 'p> {
    msg: &'m WrappedMessage,
    py: Python<'p>,
}

struct Verbatim(String);

impl Debug for Verbatim {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl<'m, 'p> Debug for DoRepr<'m, 'p> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let native = match &self.msg.0 {
            Message::None => {
                return f.debug_struct("Message").finish();
            },
            Message::Python(python) => {
                let s = match python.as_ref(self.py).repr() {
                    Ok(value) => value.to_string(),
                    Err(err) => err.to_string(),
                };
                return f.debug_tuple("Message").field(&Verbatim(s)).finish();
            },
            Message::Native(native) => native,
        };

        let mut f = f.debug_struct("Message");
        let f = f.field("type", &"native");
        match native {
            iced_native::Event::Keyboard(keyboard) => {
                let f = f
                    .field("native", &"keyboard")
                    .field("keyboard", &match keyboard {
                        iced::keyboard::Event::KeyPressed { .. } => "keyreleased",
                        iced::keyboard::Event::KeyReleased { .. } => "keyreleased",
                        iced::keyboard::Event::CharacterReceived(_) => "characterreceived",
                        iced::keyboard::Event::ModifiersChanged(_) => "modifierschanged",
                    });
                let f = match keyboard {
                    iced::keyboard::Event::KeyPressed { key_code, .. }
                    | iced::keyboard::Event::KeyReleased { key_code, .. } => {
                        f.field("key_code", key_code)
                    },
                    _ => f,
                };
                let f = match keyboard {
                    iced::keyboard::Event::KeyPressed { modifiers, .. }
                    | iced::keyboard::Event::KeyReleased { modifiers, .. }
                    | iced::keyboard::Event::ModifiersChanged(modifiers) => f
                        .field("alt", &modifiers.alt)
                        .field("control", &modifiers.control)
                        .field("logo", &modifiers.logo)
                        .field("shift", &modifiers.shift),
                    iced::keyboard::Event::CharacterReceived(c) => f.field("characterreceived", c),
                };
                f.finish()
            },

            iced_native::Event::Mouse(mouse) => {
                let f = f.field("native", &"mouse");
                let f = match mouse {
                    iced::mouse::Event::CursorEntered => f.field("mouse", &"cursorentered"),
                    iced::mouse::Event::CursorLeft => f.field("mouse", &"cursorleft"),
                    iced::mouse::Event::CursorMoved { position } => f
                        .field("mouse", &"cursormoved")
                        .field("cursormoved", &(position.x, position.y)),
                    iced::mouse::Event::ButtonPressed(b) => {
                        let f = f.field("mouse", &"buttonpressed");
                        match b {
                            iced::mouse::Button::Left => f.field("button", &"left"),
                            iced::mouse::Button::Right => f.field("button", &"right"),
                            iced::mouse::Button::Middle => f.field("button", &"middle"),
                            iced::mouse::Button::Other(i) => f.field("button", i),
                        }
                    },
                    iced::mouse::Event::ButtonReleased(_) => f.field("mouse", &"buttonreleased"),
                    iced::mouse::Event::WheelScrolled { delta } => {
                        let f = f.field("mouse", &"wheelscrolled").field(
                            "wheelscrolled",
                            &match delta {
                                iced::mouse::ScrollDelta::Lines { .. } => "lines",
                                iced::mouse::ScrollDelta::Pixels { .. } => "pixels",
                            },
                        );
                        let f = match *delta {
                            iced::mouse::ScrollDelta::Lines { x, y }
                            | iced::mouse::ScrollDelta::Pixels { x, y } => {
                                f.field("amount", &(x, y))
                            },
                        };
                        f
                    },
                };
                f.finish()
            },

            iced_native::Event::Window(window) => {
                let f = f.field("native", &"window");
                let f = match window {
                    iced_native::window::Event::Resized { width, height } => f
                        .field("window", &"resized")
                        .field("resized", &(width, height)),
                    iced_native::window::Event::CloseRequested => {
                        f.field("window", &"closerequested")
                    },
                    iced_native::window::Event::Focused => f.field("window", &"focused"),
                    iced_native::window::Event::Unfocused => f.field("window", &"unfocused"),
                    iced_native::window::Event::FileHovered(path) => {
                        f.field("window", &"fileshovered").field("file", path)
                    },
                    iced_native::window::Event::FileDropped(path) => {
                        f.field("window", &"filesdropped").field("file", path)
                    },
                    iced_native::window::Event::FilesHoveredLeft => {
                        f.field("window", &"fileshoveredleft")
                    },
                };
                f.finish()
            },

            iced_native::Event::Touch(touch) => {
                let (position, id, name) = match touch {
                    iced_native::touch::Event::FingerPressed { id, position } => {
                        (position, id, "fingerpressed")
                    },
                    iced_native::touch::Event::FingerMoved { id, position } => {
                        (position, id, "fingermoved")
                    },
                    iced_native::touch::Event::FingerLifted { id, position } => {
                        (position, id, "fingerlifted")
                    },
                    iced_native::touch::Event::FingerLost { id, position } => {
                        (position, id, "fingerlost")
                    },
                };
                f.field("native", &"touch")
                    .field("touch", &name)
                    .field("finger", &id.0)
                    .field("position", &(position.x, position.y))
                    .finish()
            },
        }
    }
}
