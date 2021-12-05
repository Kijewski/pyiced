use std::rc::Rc;

use iced::{
    executor, window, Application, Clipboard, Color, Command, Element, Font, Length, Settings,
    Space, Subscription,
};
use pyo3::exceptions::{PyAttributeError, PyRuntimeError};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::async_tasks::vec_to_command;
use crate::common::{debug_err, method_into_py, Message, ToNative};
use crate::subscriptions::{ToSubscription, WrappedSubscription};
use crate::widgets::WrappedWidgetBuilder;
use crate::wrapped::{WrappedClipboard, WrappedColor, WrappedFont};

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run_iced, m)?)?;
    Ok(())
}

#[derive(Debug, Clone)]
struct PythonApp {
    pub interop: Interop,
}

#[derive(Debug, Clone)]
pub(crate) struct Interop {
    pub new: Option<Py<PyAny>>,
    pub title: Option<Py<PyAny>>,
    pub update: Option<Py<PyAny>>,
    pub should_exit: Option<Py<PyAny>>,
    pub scale_factor: Option<Py<PyAny>>,
    pub fullscreen: Option<Py<PyAny>>,
    pub view: Option<Py<PyAny>>,
    pub subscriptions: Option<Py<PyAny>>,
    pub background_color: Option<Py<PyAny>>,

    pub put_task: Py<PyAny>,
    pub _pyloop: Py<PyAny>,
}

fn get_new_command(app: &PythonApp) -> Command<Message> {
    let new = match &app.interop.new {
        Some(new) => new,
        None => return Command::none(),
    };
    Python::with_gil(|py| {
        let vec = match new.call0(py) {
            Ok(vec) => vec,
            Err(err) => {
                err.print(py);
                return Command::none();
            },
        };
        match vec_to_command(py, vec, &app.interop.put_task) {
            Ok(command) => command,
            Err(err) => {
                err.print(py);
                Command::none()
            },
        }
    })
}

impl Application for PythonApp {
    type Executor = executor::Default;
    type Flags = Interop;
    type Message = Message;

    fn new(interop: Self::Flags) -> (PythonApp, Command<Message>) {
        let app = PythonApp { interop };
        let command = get_new_command(&app);
        (app, command)
    }

    fn subscription(&self) -> Subscription<Message> {
        let subscriptions = match &self.interop.subscriptions {
            Some(subscriptions) => subscriptions,
            None => return Subscription::none(),
        };
        Python::with_gil(|py| {
            let subscriptions = match subscriptions.call0(py) {
                Ok(subscriptions) if !subscriptions.is_none(py) => subscriptions,
                Ok(_) => return Subscription::none(),
                Err(err) => {
                    err.print(py);
                    return Subscription::none();
                },
            };
            let subscriptions = match subscriptions.as_ref(py).iter() {
                Ok(subscriptions) => subscriptions,
                Err(err) => {
                    err.print(py);
                    return Subscription::none();
                },
            };
            let subscriptions = subscriptions.filter_map(|subscription| {
                let subscription = match subscription {
                    Ok(subscription) if !subscription.is_none() => subscription,
                    Ok(_) => return None,
                    Err(err) => {
                        err.print(py);
                        return None;
                    },
                };
                let subscription = match subscription.extract() {
                    Ok(WrappedSubscription(subscription)) => subscription,
                    Err(err) => {
                        err.print(py);
                        return None;
                    },
                };
                Some(subscription.to_subscription(&self.interop))
            });
            Subscription::batch(subscriptions)
        })
    }

    fn title(&self) -> String {
        match &self.interop.title {
            Some(title) => Python::with_gil(|py| match title.call0(py) {
                Ok(s) => s.to_string(),
                Err(err) => {
                    err.print(py);
                    "<EXCEPTION>".to_owned()
                },
            }),
            None => "PyIced Application".to_owned(),
        }
    }

    fn update(&mut self, message: Message, clipboard: &mut Clipboard) -> Command<Message> {
        let (message, update) = match (message, &self.interop.update) {
            (Message::None, _) | (_, None) => return Command::none(),
            (message, Some(update)) => (message, update),
        };

        Python::with_gil(|py| {
            let vec = {
                let clipboard = Rc::new(clipboard as *mut _);
                let clipboard = WrappedClipboard(Rc::downgrade(&clipboard));
                match message {
                    m @ Message::Native(_) => update.call1(py, (m, clipboard)),
                    Message::Python(obj) => update.call1(py, (obj, clipboard)),
                    Message::None => return Command::none(), // unreachable
                }
            };
            match vec.and_then(|vec| vec_to_command(py, vec, &self.interop.put_task)) {
                Ok(command) => command,
                Err(err) => {
                    err.print(py);
                    Command::none()
                },
            }
        })
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
            Some(scale_factor) => Python::with_gil(|py| match scale_factor.call0(py) {
                Ok(s) => match s.extract(py) {
                    Ok(value) => value,
                    Err(err) => {
                        err.print(py);
                        1.0
                    },
                },
                Err(err) => {
                    err.print(py);
                    1.0
                },
            }),
            None => 1.0,
        }
    }

    fn background_color(&self) -> Color {
        match &self.interop.background_color {
            Some(background_color) => Python::with_gil(|py| match background_color.call0(py) {
                Ok(s) => match s.extract(py) {
                    Ok(WrappedColor(color)) => color,
                    Err(err) => {
                        err.print(py);
                        Color::WHITE
                    },
                },
                Err(err) => {
                    err.print(py);
                    Color::WHITE
                },
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
                    Ok(WrappedWidgetBuilder(el)) => el.to_native(py),
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

macro_rules! assign_py_to_obj {
    ($py:expr, $dest:expr, $src:expr, $name:ident $(,)?) => {
        match $src.getattr(stringify!($name)) {
            Ok(data) if !data.is_none() => $dest.$name = data.extract()?,
            Err(err) if !err.is_instance::<PyAttributeError>($py) => return Err(err),
            Ok(_) | Err(_) => {},
        }
    };

    ($py:expr, $dest:expr, $src:expr, $name:ident, $func:expr $(,)?) => {
        match $src.getattr(stringify!($name)) {
            Ok(data) if !data.is_none() => $dest.$name = $func(data.extract()?),
            Err(err) if !err.is_instance::<PyAttributeError>($py) => return Err(err),
            Ok(_) | Err(_) => {},
        }
    };
}

#[pyfunction]
pub(crate) fn run_iced(
    py: Python,
    new: &PyAny,
    title: &PyAny,
    update: &PyAny,
    should_exit: &PyAny,
    scale_factor: &PyAny,
    fullscreen: &PyAny,
    view: &PyAny,
    subscriptions: &PyAny,
    settings: &PyAny,
    background_color: &PyAny,
    taskmanager: (&PyAny, &PyAny),
) -> PyResult<()> {
    let (pyloop, put_task) = taskmanager;

    let methods = Interop {
        new: method_into_py(py, new),
        title: method_into_py(py, title),
        update: method_into_py(py, update),
        should_exit: method_into_py(py, should_exit),
        scale_factor: method_into_py(py, scale_factor),
        fullscreen: method_into_py(py, fullscreen),
        view: method_into_py(py, view),
        subscriptions: method_into_py(py, subscriptions),
        background_color: method_into_py(py, background_color),
        _pyloop: pyloop.into_py(py),
        put_task: put_task.into_py(py),
    };

    let mut settings_ = Settings {
        window: Default::default(),
        flags: methods,
        default_font: None,
        default_text_size: 20,
        exit_on_close_request: true,
        antialiasing: true,
    };

    if !settings.is_none() {
        assign_py_to_obj!(py, settings_, settings, default_text_size);
        assign_py_to_obj!(py, settings_, settings, exit_on_close_request);
        assign_py_to_obj!(py, settings_, settings, antialiasing);

        match settings.getattr("default_font") {
            Ok(data) if !data.is_none() => match data.extract()? {
                WrappedFont(Font::Default) => {},
                WrappedFont(Font::External { bytes, .. }) => {
                    settings_.default_font = Some(bytes);
                },
            },
            Err(err) if !err.is_instance::<PyAttributeError>(py) => return Err(err),
            Ok(_) | Err(_) => {},
        }

        match settings.getattr("window") {
            Ok(window) if !window.is_none() => {
                assign_py_to_obj!(py, settings_.window, window, size);
                assign_py_to_obj!(py, settings_.window, window, min_size, Some);
                assign_py_to_obj!(py, settings_.window, window, max_size, Some);
                assign_py_to_obj!(py, settings_.window, window, resizable);
                assign_py_to_obj!(py, settings_.window, window, decorations);
                assign_py_to_obj!(py, settings_.window, window, transparent);
                assign_py_to_obj!(py, settings_.window, window, always_on_top);
                // TODO: icon
            },
            Err(err) if !err.is_instance::<PyAttributeError>(py) => return Err(err),
            Ok(_) | Err(_) => {},
        }
    }

    let result =
        py.allow_threads(|| PythonApp::run(settings_).map_err(debug_err::<PyRuntimeError, _>));
    if let Err(err) = put_task.call0() {
        err.print(py);
    }
    result?;

    // Impossible: iced::Application::run only returns on error.
    Err(PyErr::new::<PyRuntimeError, _>(
        "Could not start application.",
    ))
}
