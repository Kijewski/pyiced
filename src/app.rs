use std::pin::Pin;
use std::sync::atomic::AtomicUsize;

use futures_util::Future;
use iced::{
    executor, window, Application, Clipboard, Color, Command, Element, Length, Settings, Space,
    Subscription,
};
use pyo3::exceptions::{PyAttributeError, PyException, PyTypeError};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use tokio::sync::oneshot::{self, Sender};

use crate::common::{method_into_py, Message, ToNative};
use crate::subscriptions::{ToSubscription, WrappedSubscription};
use crate::widgets::WrappedWidgetBuilder;
use crate::wrapped::{WrappedColor, WrappedMessage};

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run_iced, m)?)?;
    Ok(())
}

#[derive(Debug, Clone)]
struct PythonApp {
    pub interop: Interop,
}

#[derive(Debug, Clone)]
struct Interop {
    pub new: Option<Py<PyAny>>,
    pub title: Option<Py<PyAny>>,
    pub update: Option<Py<PyAny>>,
    pub should_exit: Option<Py<PyAny>>,
    pub scale_factor: Option<Py<PyAny>>,
    pub fullscreen: Option<Py<PyAny>>,
    pub view: Option<Py<PyAny>>,
    pub subscriptions: Option<Py<PyAny>>,
    pub background_color: Option<Py<PyAny>>,

    pub pyloop: Py<PyAny>,
    pub put_task: Py<PyAny>,
}

enum MessageOrFuture {
    Message(Message),
    Future(Pin<Box<dyn Future<Output = Py<PyAny>> + Send>>),
    None,
}

static INDEX: AtomicUsize = AtomicUsize::new(0);

#[pyclass]
struct Task {
    #[pyo3(get)]
    pub task: Py<PyAny>,

    #[pyo3(set)]
    pub result: Py<PyAny>,
    
    pub done: Option<Sender<Py<PyAny>>>,
}

#[pymethods]
impl Task {
    #[call]
    fn __call__(&mut self) {
        if let Some(done) = self.done.take() {
            done.send(self.result.clone()).expect("Channel broken: send");
        }
    }
}

fn message_or_future(py: Python, result: PyResult<&PyAny>, app: &PythonApp) -> MessageOrFuture {
    let result = match result {
        Ok(result) => result,
        Err(err) => {
            err.print(py);
            return MessageOrFuture::None;
        }
    };
    if result.is_none() {
        return MessageOrFuture::None;
    }
    if let Ok(WrappedMessage(msg)) = result.extract() {
        return MessageOrFuture::Message(msg)
    }

    let index = INDEX.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

    eprintln!("A index={} thread={:?} line={} repr={:#}", index, std::thread::current().id(), line!(), result.repr().unwrap());

    let (sender, receiver) = oneshot::channel();
    let task = Task {
        task: result.into_py(py),
        result: py.None(),
        done: Some(sender),
    };
    if let Err(err) = app.interop.put_task.call1(py, (task,)) {
        err.print(py);
        return MessageOrFuture::None;
    }

    MessageOrFuture::Future(Box::pin(async move {
        eprintln!("B index={} thread={:?} line={}", index, std::thread::current().id(), line!());
        let result = receiver.await.expect("Channel broken: recv");
        eprintln!("C index={} thread={:?} line={}", index, std::thread::current().id(), line!());
        return result;
    }))
}

fn future_to_command(future: Pin<Box<dyn Future<Output = Py<PyAny>> + Send>>) -> Command<Message> {
    Command::perform(future, |result| {
        Python::with_gil(|py| {
            let result = result.as_ref(py);
            let result = match result.extract::<(Option<&PyAny>, Option<&PyAny>)>() {
                Ok(result) => result,
                Err(err) => {
                    PyErr::new::<PyTypeError, _>(format!("{:?}", err)).print(py);
                    return Message::None;
                },
            };
            let result = match result {
                (Some(err), _) => {
                    PyErr::from_instance(err).print(py);
                    return Message::None;
                }
                (None, None) => {
                    return Message::None;
                }
                (None, Some(result)) => result,
            };
            match result.extract() {
                Ok(WrappedMessage(msg)) => {
                    msg
                },
                Err(err) => {
                    err.print(py);
                    Message::None
                },
            }
        })
    })
}

fn get_new_command(app: &PythonApp) -> Command<Message> {
    let new = match &app.interop.new {
        Some(new) => new,
        None => return Command::none(),
    };

    let msg = Python::with_gil(|py| {
        let datum = new.call0(py);
        let datum = match datum {
            Ok(ref datum) => Ok(datum.as_ref(py)),
            Err(err) => Err(err),
        };
        message_or_future(py, datum, &app)
    });

    match msg {
        MessageOrFuture::Message(msg) => Command::from(async { msg }),
        MessageOrFuture::Future(future) => future_to_command(future),
        MessageOrFuture::None => Command::none(),
    }
}

/*
fn start_loop(py: Python, sender: SyncSender<Result<Py<PyAny>, PyErr>>) {
    let main = || -> PyResult<_> {
        let asyncio = py.import("asyncio")?;
        let new_event_loop = asyncio.getattr("new_event_loop")?;
        let set_event_loop = asyncio.getattr("set_event_loop")?;
        let event = asyncio.getattr("Event")?;

        let pyloop = new_event_loop.call0()?;
        set_event_loop.call1((pyloop,))?;

        let done_event = event.call0()?;

        let main = done_event.getattr("wait")?;
        let run_until_complete = pyloop.getattr("run_until_complete")?;

        sender.send(Ok(pyloop.into_py(py))).unwrap();

        run_until_complete.call1((main,))?;

        Ok(py.None().into_py(py))
    };
    let result = main();
    sender.send(result).expect("Send channel failed");
}
*/

impl Application for PythonApp {
    type Executor = executor::Default;
    type Flags = Interop;
    type Message = Message;

    fn new(interop: Self::Flags) -> (PythonApp, Command<Message>) {
        // let (sender, receiver) = sync_channel(1);
        // spawn(move || Python::with_gil(|py| start_loop(py, sender)));
        // let pyloop = receiver.recv().expect("Recv channel failed").unwrap();

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
                let subscription = match subscription.extract::<WrappedSubscription>() {
                    Ok(subscription) => subscription.0,
                    Err(err) => {
                        err.print(py);
                        return None;
                    },
                };
                Some(subscription.to_subscription())
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

    fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        let (message, update) = match (message, &self.interop.update) {
            (Message::None, _) | (_, None) => return Command::none(),
            (message, Some(update)) => (message, update),
        };

        Python::with_gil(|py| {
            let vec = PyCell::new(py, WrappedMessage(message))
                .and_then(|message| update.call1(py, (message,)));
            let vec = match vec {
                Ok(vec) => vec,
                Err(err) => {
                    err.print(py);
                    return Command::none();
                },
            };

            let vec = vec.as_ref(py);
            if vec.is_none() {
                return Command::none();
            }
            if let Ok(WrappedMessage(msg)) = vec.extract() {
                return Command::from(async { msg });
            }

            let iter = match vec.iter() {
                Ok(iter) => iter,
                Err(err) => {
                    err.print(py);
                    return Command::none();
                }
            };

            let mut commands = Vec::new();
            for datum in iter {
                let datum = message_or_future(py, datum, &self);
                let command = match datum {
                    MessageOrFuture::Message(msg) => {
                        Command::from(async move { msg })
                    },
                    MessageOrFuture::Future(future) => {
                        future_to_command(future)
                    }
                    MessageOrFuture::None => continue,
                };
                commands.push(command)
            }

            Command::batch(commands)
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
                Ok(s) => match s.extract::<WrappedColor>(py) {
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
    eprintln!("run_iced index={} thread={:?} line={}", 0, std::thread::current().id(), line!());

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
        pyloop: pyloop.into_py(py),
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

    py.allow_threads(|| {
        eprintln!("run_iced index={} thread={:?} line={}", 0, std::thread::current().id(), line!());
        PythonApp::run(settings_).map_err(|err| PyException::new_err(format!("{}", err)))
    })
}
