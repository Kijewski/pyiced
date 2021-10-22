use pyo3::prelude::*;
use pyo3::{PyGCProtocol, PyTraverseError, PyVisit};

use crate::common::{GCProtocol, Message};

macro_rules! init_mod {
    ( $($module:ident -> $typ:ident),* $(,)? ) => {
        $( mod $module; )*

        $( pub(crate) use $module::$typ; )*

        #[derive(Debug, Clone)]
        pub(crate) enum Subscription {
            $( $typ($typ) ),*
        }

        impl GCProtocol for Subscription {
            fn traverse(&self, visit: &PyVisit) -> Result<(), PyTraverseError> {
                match self {
                    $( Subscription::$typ(value) => value.traverse(visit) ),*
                }
            }
        }

        impl ToSubscription for Subscription {
            fn to_subscription(&self) -> iced_native::Subscription<Message> {
                match self {
                    $( Subscription::$typ(value) => value.to_subscription() ),*
                }
            }
        }

        $(
            impl From<$typ> for Subscription {
                fn from(value: $typ) -> Subscription {
                    Subscription::$typ(value)
                }
            }

            impl From<$typ> for WrappedSubscription {
                fn from(value: $typ) -> WrappedSubscription {
                    WrappedSubscription(Subscription::$typ(value), Private)
                }
            }
        )*
    };
}

init_mod! {
    no_subscription -> NoSubscription,
    uncaptured -> Uncaptured,
}

pub(crate) trait ToSubscription {
    fn to_subscription(&self) -> iced::Subscription<Message>;
}

impl Default for Subscription {
    fn default() -> Self {
        Self::NoSubscription(NoSubscription)
    }
}

pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedSubscription>()?;
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Private;

#[pyclass(name = "Subscription", module = "pyiced")]
#[derive(Debug, Clone)]
pub(crate) struct WrappedSubscription(pub Subscription, Private);

#[pyproto]
impl PyGCProtocol for WrappedSubscription {
    fn __traverse__(&self, visit: pyo3::PyVisit) -> Result<(), pyo3::PyTraverseError> {
        self.0.traverse(&visit)?;
        Ok(())
    }

    fn __clear__(&mut self) {
        self.0 = Default::default();
    }
}

#[allow(non_snake_case)]
#[pymethods]
impl WrappedSubscription {
    #[classattr]
    fn NONE() -> Self {
        NoSubscription.into()
    }

    #[classattr]
    fn UNCAPTURED() -> Self {
        Uncaptured.into()
    }
}
