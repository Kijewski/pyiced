use iced::{
    executor, window, Application, Clipboard, Color, Command, Element, Length, Settings, Space,
    Subscription,
};
use iced_native::subscription::events;
use pyo3::exceptions::{PyAttributeError, PyException};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::common::{method_into_py, py_to_command, Message, ToNative};
use crate::widgets::WrappedWidgetBuilder;
use crate::wrapped::{WrappedColor, WrappedMessage};

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run_iced, m)?)?;
    Ok(())
}

#[derive(Debug, Clone)]
struct PythonApp {
    interop: Interop,
}

#[derive(Debug, Clone)]
struct Interop {
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
    type Flags = Interop;
    type Message = Message;

    fn new(interop: Self::Flags) -> (PythonApp, Command<Message>) {
        let app = PythonApp { interop };

        let command = match &app.interop.new {
            Some(new) => {
                Python::with_gil(|py| py_to_command(py, &app.interop.pyloop, new.call0(py)))
            },
            None => Command::none(),
        };

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
                    err.print(py);
                    "<EXCEPTION>".to_owned()
                },
            }),
            None => "PyIced Application".to_owned(),
        }
    }

    fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        match (message, &self.interop.update) {
            (Message::None, _) | (_, None) => Command::none(),
            (message, Some(update)) => Python::with_gil(|py| {
                let vec = PyCell::new(py, WrappedMessage(message))
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
            Some(scale_factor) => Python::with_gil(|py| match scale_factor.call0(py) {
                Ok(s) => match s.as_ref(py).extract() {
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
        match &self.interop.scale_factor {
            Some(scale_factor) => Python::with_gil(|py| match scale_factor.call0(py) {
                Ok(s) => match s.as_ref(py).extract::<WrappedColor>() {
                    Ok(color) => color.0,
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
    settings: &'a PyAny,
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
        // TODO: default_font

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

    PythonApp::run(settings_).map_err(|err| PyException::new_err(format!("{}", err)))
}
