use futures_util::FutureExt;
use iced::{Application, Clipboard, Color, Command, Element, Length, Settings, Space, Subscription, executor, window};
use iced_native::subscription::events;
use pyo3::exceptions::PyException;
use pyo3::types::{PyFloat, PyList};
use {pyo3::prelude::*, wrap_pyfunction};
use pyo3_asyncio::into_future_with_loop;

use crate::to_native::ToNative;
use crate::wrapped;

pub(crate) fn init(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run_iced, m)?)?;
    Ok(())
}

#[derive(Debug, Clone)]
pub(crate) enum Message {
    None,
    Native(iced_native::Event),
    Python(Py<PyAny>),
}

#[derive(Debug, Clone)]
pub(crate) struct PythonApp {
    interop: Interop,
}

#[derive(Debug, Clone)]
pub(crate) struct Interop {
    pub pyloop: PyObject,
    pub new: Option<Py<PyAny>>,
    pub title: Option<Py<PyAny>>,
    pub update: Option<Py<PyAny>>,
    pub should_exit: Option<Py<PyAny>>,
    pub scale_factor: Option<Py<PyAny>>,
    pub fullscreen: Option<Py<PyAny>>,
    pub view: Option<Py<PyAny>>,
}

impl<'a> Application for PythonApp {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = Interop;

    fn new(interop: Self::Flags) -> (PythonApp, Command<Message>) {
        let app = PythonApp { interop };

        println!("line={}", line!());
        let command = match &app.interop.new {
            Some(new) => Python::with_gil(|py| {
                println!("line={}", line!());
                py_to_command(py, &app.interop.pyloop, new.call0(py))
            }),
            None => Command::none(),
        };
        println!("line={}", line!());

        (app, command)
    }

    fn subscription(&self) -> Subscription<Message> {
        events().map(Message::Native)
    }

    fn title(&self) -> String {
        match &self.interop.title {
            Some(title) => Python::with_gil(|py| match title.call0(py) {
                Ok(s) => s.to_string(),
                Err(err) => {
                    println!("line={}", line!());
                    err.print(py);
                    "<EXCEPTION>".to_owned()
                }
            }),
            None => "PyIced Application".to_owned(),
        }
    }

    fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        dbg!(&message);
        match (message, &self.interop.update) {
            (Message::None, _) | (_, None) => Command::none(),
            (message, Some(update)) => Python::with_gil(|py| {
                let vec = PyCell::new(py, wrapped::Message(message))
                    .and_then(|message| update.call1(py, (message,)));
                py_to_command(py, &self.interop.pyloop, vec)
            }),
        }
    }

    fn should_exit(&self) -> bool {
        match &self.interop.should_exit {
            Some(should_exit) => Python::with_gil(|py| {
                let err = match should_exit.call0(py) {
                    Ok(s) => match s.is_true(py) {
                        Ok(value) => return value,
                        Err(err) => err,
                    },
                    Err(err) => err,
                };
                err.print(py);
                false
            }),
            None => false,
        }
    }

    fn scale_factor(&self) -> f64 {
        match &self.interop.scale_factor {
            Some(scale_factor) => Python::with_gil(|py| {
                match scale_factor.call0(py) {
                    Ok(s) => match s.as_ref(py).downcast::<PyFloat>() {
                        Ok(value) => return value.value(),
                        Err(err) => {
                            dbg!(err); // TODO
                            1.0
                        },
                    },
                    Err(err) => {
                        err.print(py);
                        1.0
                    },
                }
            }),
            None => 1.0,
        }
    }

    fn background_color(&self) -> Color {
        match &self.interop.scale_factor {
            Some(scale_factor) => Python::with_gil(|py| {
                match scale_factor.call0(py) {
                    Ok(s) => match s.as_ref(py).extract() {
                        Ok(wrapped::Color(value)) => return value,
                        Err(err) => {
                            dbg!(err); // TODO
                            Color::WHITE
                        },
                    },
                    Err(err) => {
                        err.print(py);
                        Color::WHITE
                    },
                }
            }),
            None => Color::WHITE,
        }
    }

    fn mode(&self) -> window::Mode {
        match &self.interop.fullscreen {
            Some(fullscreen) => Python::with_gil(|py| {
                let err = match fullscreen.call0(py) {
                    Ok(s) => match s.is_true(py) {
                        Ok(true) => return window::Mode::Fullscreen,
                        Ok(false) => return window::Mode::Windowed,
                        Err(err) => err,
                    },
                    Err(err) => err,
                };
                err.print(py);
                window::Mode::Windowed
            }),
            None => window::Mode::Windowed,
        }
    }

    fn view(&mut self) -> Element<Message> {
        match &self.interop.view {
            Some(view) => Python::with_gil(|py| match view.call0(py) {
                Ok(el) if !el.is_none(py) => match el.extract(py) {
                    Ok(wrapped::Element(el)) => el.to_native(py),
                    Err(err) => {
                        err.print(py);
                        Space::new(Length::Shrink, Length::Shrink).into()
                    },
                },
                Ok(_) => Space::new(Length::Shrink, Length::Shrink).into(),
                Err(err) => {
                    err.print(py);
                    Space::new(Length::Shrink, Length::Shrink).into()
                },
            }),
            None => Space::new(Length::Shrink, Length::Shrink).into(),
        }
    }
}

fn py_to_command(py: Python, pyloop: &Py<PyAny>, vec: PyResult<PyObject>) -> Command<Message> {
    match vec {
        Ok(vec) if !vec.is_none(py) => match vec.as_ref(py).downcast::<PyList>() {
            Ok(vec) if !vec.is_none() && !vec.is_empty() => {
                let vec = vec.into_iter().map(|command| {
                    match into_future_with_loop(pyloop.as_ref(py), command) {
                        Ok(fut) => Command::from({
                            fut.map(|result| {
                                Python::with_gil(|py| match result {
                                    Ok(msg) if !msg.is_none(py) => match msg.as_ref(py).extract() {
                                        Ok(wrapped::Message(msg)) => return msg,
                                        Err(err) => {
                                            err.print(py);
                                            Message::None
                                        }
                                    },
                                    Ok(_) => Message::None,
                                    Err(err) => {
                                        err.print(py);
                                        Message::None
                                    }
                                })
                            })
                        }),
                        Err(err) => {
                            Python::with_gil(|py| err.print(py));
                            Command::from(async { Message::None })
                        }
                    }
                });
                Command::batch(vec)
            }
            Ok(_) => Command::none(),
            Err(err) => {
                dbg!(err); // TODO
                Command::none()
            }
        },
        Ok(_) => Command::none(),
        Err(err) => {
            err.print(py);
            Command::none()
        }
    }
}

fn method_into_py(py: Python, method: &PyAny) -> Option<Py<PyAny>> {
    match method.is_none() {
        false => Some(method.into_py(py)),
        true => None,
    }
}

#[pyfunction]
pub(crate) fn run_iced<'a>(
    py: Python,
    pyloop: &'a PyAny,
    new: &'a PyAny,
    title: &'a PyAny,
    update: &'a PyAny,
    should_exit: &'a PyAny,
    scale_factor: &'a PyAny,
    fullscreen: &'a PyAny,
    view: &'a PyAny,
) -> PyResult<()> {
    let methods = Interop {
        pyloop: pyloop.into_py(py),
        new: method_into_py(py, new),
        title: method_into_py(py, title),
        update: method_into_py(py, update),
        should_exit: method_into_py(py, should_exit),
        scale_factor: method_into_py(py, scale_factor),
        fullscreen: method_into_py(py, fullscreen),
        view: method_into_py(py, view),
    };
    let settings = Settings {
        window: Default::default(),
        flags: methods,
        default_font: None,
        default_text_size: 20,
        exit_on_close_request: true,
        antialiasing: true,
    };
    eprintln!("line={}", line!());
    PythonApp::run(settings).map_err(|err| PyException::new_err(format!("{}", err)))
}
