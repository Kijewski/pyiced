use std::fmt::{Debug, Write};
use std::path::PathBuf;

use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use pyo3::{PyGCProtocol, PyTraverseError, PyVisit};

use crate::common::{debug_str, EitherPy, Message};
use crate::format_to_py;

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedMessage>()?;
    Ok(())
}

/// A message generated through user interaction.
///
/// Messages get passed to to :meth:`~pyiced.IcedApp.update()`.
#[pyclass(name = "Message", module = "pyiced", gc)]
#[derive(Debug, Default, Clone)]
struct WrappedMessage(pub Message);

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

impl IntoPy<Py<PyAny>> for Message {
    fn into_py(self, py: Python<'_>) -> Py<PyAny> {
        WrappedMessage(self).into_py(py)
    }
}

#[pymethods]
impl WrappedMessage {
    /// The python object passed to the constructor.
    ///
    /// Returns
    /// -------
    /// Optional[object]
    ///     None for "native" message, otherwise the object.
    #[getter]
    fn _python(&self) -> Option<Py<PyAny>> {
        match &self.0 {
            Message::Python(value) => Some(value.clone()),
            _ => None,
        }
    }

    /// The kind of the native message.
    ///
    /// Returns
    /// -------
    /// str
    ///     * `"mouse"` for mouse interactions, e.g. mouse clicking
    ///     * `"window"` for window interactions, e.g. resizing
    ///     * `"keyboard"` for keyboard interactions, e.g. key presses
    ///     * `"touch"` for touch interactions (impossible?)
    #[getter]
    fn kind(&self) -> Option<&'static str> {
        Some(match get_native(self).ok()? {
            iced_native::Event::Mouse(_) => "mouse",
            iced_native::Event::Window(_) => "window",
            iced_native::Event::Touch(_) => "touch",
            iced_native::Event::Keyboard(_) => "keyboard",
        })
    }

    // keyboard

    /// The kind of the keyboard interaction.
    ///
    /// Returns
    /// -------
    /// Optional[str]
    ///     * `None` if not a ``Message(native="keyboard")`` interaction
    ///     * `"keypressed"` when a key was pushed down
    ///     * `"keyreleased"` when a key no more pushed down
    ///     * `"characterreceived"` when a key press + release generated a character
    ///     * `"modifierschanged"` when a modifier was pressed or released, e.g. shift
    #[getter]
    fn keyboard(&self) -> Option<&'static str> {
        Some(match get_keyboard(self).ok()? {
            iced::keyboard::Event::KeyPressed { .. } => "keypressed",
            iced::keyboard::Event::KeyReleased { .. } => "keyreleased",
            iced::keyboard::Event::CharacterReceived(_) => "characterreceived",
            iced::keyboard::Event::ModifiersChanged(_) => "modifierschanged",
        })
    }

    /// The name of the pressed or released key.
    ///
    /// See `iced_native::keyboard::KeyCode <https://docs.rs/iced_native/0.4.0/iced_native/keyboard/enum.KeyCode.html>`_ for the name of the keys.
    ///
    /// Returns
    /// -------
    /// Optional[str]
    ///     The name of the key, or `None` if the not a key press or release.
    #[getter]
    fn key_code(&self) -> PyResult<Option<String>> {
        let c = match get_keyboard(self) {
            Ok(c) => c,
            Err(_) => return Ok(None),
        };
        match c {
            iced::keyboard::Event::KeyPressed { key_code, .. }
            | iced::keyboard::Event::KeyReleased { key_code, .. } => {
                Ok(Some(format_to_py!("{:?}", key_code)?))
            },
            _ => Ok(None),
        }
    }

    /// The shift key was pressed / released.
    ///
    /// Returns
    /// -------
    /// Optional[bool]
    ///     * `True` – Yes, the shift key was pressed or released.
    ///     * `False` – No, the state of the shift key is unchanged.
    ///     * `None` – Not a `"keypress"`, `"keyrelease"` or `"modifierschanged"` event.
    #[getter]
    fn shift(&self) -> Option<bool> {
        match get_keyboard(self).ok()? {
            iced::keyboard::Event::KeyPressed { modifiers, .. }
            | iced::keyboard::Event::KeyReleased { modifiers, .. } => Some(modifiers.shift),
            _ => None,
        }
    }

    /// The alt key was pressed / released.
    ///
    /// Returns
    /// -------
    /// Optional[bool]
    ///     * `True` – Yes, the alt key was pressed or released.
    ///     * `False` – No, the state of the alt key is unchanged.
    ///     * `None` – Not a `"keypress"`, `"keyrelease"` or `"modifierschanged"` event.
    #[getter]
    fn alt(&self) -> Option<bool> {
        match get_keyboard(self).ok()? {
            iced::keyboard::Event::KeyPressed { modifiers, .. }
            | iced::keyboard::Event::KeyReleased { modifiers, .. } => Some(modifiers.alt),
            _ => None,
        }
    }

    /// The "logo" key was pressed / released.
    ///
    /// The logo key is the windows key, command key, etc.
    ///
    /// Returns
    /// -------
    /// Optional[bool]
    ///     * `True` – Yes, the "logo" key was pressed or released.
    ///     * `False` – No, the state of the "logo" key is unchanged.
    ///     * `None` – Not a `"keypress"`, `"keyrelease"` or `"modifierschanged"` event.
    #[getter]
    fn logo(&self) -> Option<bool> {
        match get_keyboard(self).ok()? {
            iced::keyboard::Event::KeyPressed { modifiers, .. }
            | iced::keyboard::Event::KeyReleased { modifiers, .. }
            | iced::keyboard::Event::ModifiersChanged(modifiers) => Some(modifiers.logo),
            _ => None,
        }
    }

    /// The control key was pressed / released.
    ///
    /// Returns
    /// -------
    /// Optional[bool]
    ///     * `True` – Yes, the control key was pressed or released.
    ///     * `False` – No, the state of the control key is unchanged.
    ///     * `None` – Not a `"keypress"`, `"keyrelease"` or `"modifierschanged"` event.
    #[getter]
    fn control(&self) -> Option<bool> {
        match get_keyboard(self).ok()? {
            iced::keyboard::Event::KeyPressed { modifiers, .. }
            | iced::keyboard::Event::KeyReleased { modifiers, .. }
            | iced::keyboard::Event::ModifiersChanged(modifiers) => Some(modifiers.control),
            _ => None,
        }
    }

    /// A unicode character was received.
    ///
    /// Returns
    /// -------
    /// Optional[str]
    ///     The received, composed character. `None` if not a character event.
    #[getter]
    fn characterreceived(&self) -> PyResult<Option<String>> {
        let c = match get_keyboard(self) {
            Ok(c) => c,
            Err(_) => return Ok(None),
        };
        match c {
            iced::keyboard::Event::CharacterReceived(c) => Ok(Some(format_to_py!("{}", c)?)),
            _ => Ok(None),
        }
    }

    // mouse

    /// A mouse event.
    ///
    /// Returns
    /// -------
    /// Optional[str]
    ///     * `"cursorentered"` – The mouse cursor entered the window.
    ///     * `"cursorleft"` – The mouse cursor left the window.
    ///     * `"cursormoved"` – The mouse cursor was moved
    ///     * `"buttonpressed"` – A mouse button was pressed.
    ///     * `"buttonreleased"` – A mouse button was released.
    ///     * `"wheelscrolled"` – The mouse wheel was scrolled.
    ///     * `None` – Not a mouse event.
    #[getter]
    fn mouse(&self) -> Option<&'static str> {
        Some(match get_mouse(self).ok()? {
            iced::mouse::Event::CursorEntered => "cursorentered",
            iced::mouse::Event::CursorLeft => "cursorleft",
            iced::mouse::Event::CursorMoved { .. } => "cursormoved",
            iced::mouse::Event::ButtonPressed(_) => "buttonpressed",
            iced::mouse::Event::ButtonReleased(_) => "buttonreleased",
            iced::mouse::Event::WheelScrolled { .. } => "wheelscrolled",
        })
    }

    /// The mouse cursor was moved
    ///
    /// Returns
    /// -------
    /// Optional[tuple[float, float]]
    ///     Horizontal and vertical pixels, or `None` if cursor was not moved.
    #[getter]
    fn cursormoved(&self) -> Option<(f32, f32)> {
        match get_mouse(self).ok()? {
            iced::mouse::Event::CursorMoved { position } => Some((position.x, position.y)),
            _ => None,
        }
    }

    /// The button of a mouse event.
    ///
    /// Returns
    /// -------
    /// Union[str|int|None]
    ///     * `"left"` – The left mouse button.
    ///     * `"right"` – The right mouse button.
    ///     * `"middle"` – The middle (wheel) button.
    ///     * :class:`int` – Another button.
    ///     * `None` – Not a mouse event.
    #[getter]
    fn button(&self) -> Option<EitherPy<&'static str, u32>> {
        let b = match get_mouse(self).ok()? {
            iced::mouse::Event::ButtonPressed(b) | iced::mouse::Event::ButtonReleased(b) => b,
            _ => return None,
        };
        Some(match b {
            iced::mouse::Button::Left => EitherPy::Left("left"),
            iced::mouse::Button::Right => EitherPy::Left("right"),
            iced::mouse::Button::Middle => EitherPy::Left("middle"),
            iced::mouse::Button::Other(i) => EitherPy::Right(*i as u32),
        })
    }

    /// The unit of the scroll movement.
    ///
    /// Returns
    /// -------
    /// Optional[str]
    ///     * `"lines"` – Counting in lines / columns.
    ///     * `"pixels"` – Counting in pixels.
    ///     * `None` – Not a scroll movement.
    #[getter]
    fn wheelscrolled(&self) -> Option<&'static str> {
        let delta = match get_mouse(self).ok()? {
            iced::mouse::Event::WheelScrolled { delta } => delta,
            _ => return None,
        };
        Some(match delta {
            iced::mouse::ScrollDelta::Lines { .. } => "lines",
            iced::mouse::ScrollDelta::Pixels { .. } => "pixels",
        })
    }

    /// The scroll movement.
    ///
    /// Returns
    /// -------
    /// Optional[tuple[float, float]]
    ///     The horizontal and vertical amount.
    ///     The unit can be determined by inspecting :attr:`~pyiced.Message.wheelscrolled`.
    ///
    ///     `None` if not a scroll movement.
    #[getter]
    fn amount(&self) -> Option<(f32, f32)> {
        let delta = match get_mouse(self).ok()? {
            iced::mouse::Event::WheelScrolled { delta } => delta,
            _ => return None,
        };
        Some(match delta {
            iced::mouse::ScrollDelta::Lines { x, y }
            | iced::mouse::ScrollDelta::Pixels { x, y } => (*x, *y),
        })
    }

    // touch

    /// A touch interaction.
    ///
    /// Returns
    /// -------
    /// Optional[str]
    ///     * `"fingerpressed"` – A touch interaction was started.
    ///     * `"fingermoved"` – An on-going touch interaction was moved.
    ///     * `"fingerlifted"` – A touch interaction was ended.
    ///     * `"fingerlost"` – A touch interaction was canceled.
    ///     * `None` – Not a touch interaction.
    #[getter]
    fn touch(&self) -> Option<&'static str> {
        Some(match get_touch(self).ok()? {
            iced_native::touch::Event::FingerPressed { .. } => "fingerpressed",
            iced_native::touch::Event::FingerMoved { .. } => "fingermoved",
            iced_native::touch::Event::FingerLifted { .. } => "fingerlifted",
            iced_native::touch::Event::FingerLost { .. } => "fingerlost",
        })
    }

    /// A unique identifier representing a finger on a touch interaction.
    ///
    /// Returns
    /// -------
    /// int
    ///     Identifier of the finger.
    #[getter]
    fn finger(&self) -> Option<u64> {
        Some(match get_touch(self).ok()? {
            iced_native::touch::Event::FingerPressed { id, .. }
            | iced_native::touch::Event::FingerMoved { id, .. }
            | iced_native::touch::Event::FingerLifted { id, .. }
            | iced_native::touch::Event::FingerLost { id, .. } => id.0,
        })
    }

    /// A 2D point for the touch interaction.
    ///
    /// Returns
    /// -------
    /// tuple[float, float]
    ///     A 2D point
    #[getter]
    fn position(&self) -> Option<(f32, f32)> {
        Some(match get_touch(self).ok()? {
            iced_native::touch::Event::FingerPressed { position, .. }
            | iced_native::touch::Event::FingerMoved { position, .. }
            | iced_native::touch::Event::FingerLifted { position, .. }
            | iced_native::touch::Event::FingerLost { position, .. } => (position.x, position.y),
        })
    }

    // window

    /// The kind of the window message.
    ///
    /// Returns
    /// -------
    /// Optional[str]
    ///     * `"resized"` – The window was resized.
    ///     * `"closerequested"` – The window close button was clicked.
    ///     * `"focused"` – The window was focus.
    ///     * `"unfocused"` – The focus left the window.
    ///     * `"filehovered"` – A file is hovering the window. A selection of multiple files cause multiple messages.
    ///     * `"filedropped"` – A file was dropped on the window. A selection of multiple files cause multiple messages.
    ///     * `"fileshoveredleft"` – The cursor the with hovering file(s) left the window.
    ///     * `None` – Not a window message.
    #[getter]
    fn window(&self) -> Option<&'static str> {
        Some(match get_window(self).ok()? {
            iced_native::window::Event::Resized { .. } => "resized",
            iced_native::window::Event::CloseRequested => "closerequested",
            iced_native::window::Event::Focused => "focused",
            iced_native::window::Event::Unfocused => "unfocused",
            iced_native::window::Event::FileHovered(_) => "filehovered",
            iced_native::window::Event::FileDropped(_) => "filedropped",
            iced_native::window::Event::FilesHoveredLeft => "fileshoveredleft",
        })
    }

    /// The new size of the window.
    ///
    /// Returns
    /// -------
    /// Optional[tuple(int, int)]
    ///     The width and height in pixels or null, if it's not a resize action.
    #[getter]
    fn resized(&self) -> Option<(u32, u32)> {
        match get_window(self).ok()? {
            iced_native::window::Event::Resized { width, height } => Some((*width, *height)),
            _ => None,
        }
    }

    /// The path of the hovering or dropped file.
    ///
    /// Returns
    /// -------
    /// Optional[pathlib.Path]
    ///     The path or none, if the Message is not a file action.
    #[getter]
    fn file(&self) -> Option<PathBuf> {
        match get_window(self).ok()? {
            iced_native::window::Event::FileHovered(path)
            | iced_native::window::Event::FileDropped(path) => Some(path.clone()),
            _ => None,
        }
    }

    #[classattr]
    #[allow(non_snake_case)]
    fn __match_args__() -> (&'static str,) {
        // Structural Pattern Matching: https://www.python.org/dev/peps/pep-0634/
        ("_python",)
    }

    fn __str__(&self) -> PyResult<String> {
        debug_str(&self.0)
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
        match native {
            iced_native::Event::Keyboard(keyboard) => {
                let f = f
                    .field("kind", &"keyboard")
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
                let f = f.field("kind", &"mouse");
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
                let f = f.field("kind", &"window");
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
                f.field("kind", &"touch")
                    .field("touch", &name)
                    .field("finger", &id.0)
                    .field("position", &(position.x, position.y))
                    .finish()
            },
        }
    }
}

#[derive(Debug, Default, Clone)]
pub(crate) struct MessageOrDatum(pub Message);

impl<'source> FromPyObject<'source> for MessageOrDatum {
    fn extract(ob: &'source PyAny) -> PyResult<Self> {
        let datum = match ob.extract::<Option<EitherPy<WrappedMessage, Py<PyAny>>>>()? {
            None => return Ok(MessageOrDatum(Message::None)),
            Some(datum) => datum,
        };
        match datum {
            EitherPy::Left(WrappedMessage(message)) => Ok(MessageOrDatum(message)),
            EitherPy::Right(datum) => Ok(MessageOrDatum(Message::Python(datum))),
        }
    }
}

impl From<MessageOrDatum> for Message {
    fn from(m: MessageOrDatum) -> Message {
        m.0
    }
}

impl From<MessageOrDatum> for Option<Message> {
    fn from(m: MessageOrDatum) -> Option<Message> {
        match m.0 {
            Message::None => None,
            m => Some(m),
        }
    }
}
