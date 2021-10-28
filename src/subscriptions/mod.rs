use pyo3::prelude::*;
use pyo3::PyGCProtocol;

use crate::common::{GCProtocol, Message};

macro_rules! init_mod {
    ( $($module:ident -> $typ:ident),* $(,)? ) => {
        $( mod $module; )*

        $( pub(crate) use $module::$typ; )*

        pub(crate) fn init_mod(py: Python, m: &PyModule) -> PyResult<()> {
            m.add_class::<WrappedSubscription>()?;
            $(
                {
                    use $module::init_mod;
                    init_mod(py, m)?;
                }
            )*
            Ok(())
        }

        #[derive(Debug, Clone)]
        pub(crate) enum Subscription {
            $( $typ($typ) ),*
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
                    Self(Subscription::$typ(value), Private)
                }
            }
        )*

        #[pyproto]
        impl PyGCProtocol for WrappedSubscription {
            fn __traverse__(&self, visit: pyo3::PyVisit) -> Result<(), pyo3::PyTraverseError> {
                match &self.0 {
                    $( Subscription::$typ(value) => value.traverse(&visit) ),*
                }
            }

            fn __clear__(&mut self) {
                self.0 = Default::default();
            }
        }
    };
}

init_mod! {
    no_subscription -> NoSubscription,
    uncaptured -> Uncaptured,
    every -> Every,
}

pub(crate) trait ToSubscription {
    fn to_subscription(&self) -> iced::Subscription<Message>;
}

impl Default for Subscription {
    fn default() -> Self {
        Self::NoSubscription(NoSubscription)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Private;

/// TODO
#[pyclass(name = "Subscription", module = "pyiced")]
#[derive(Debug, Clone)]
pub(crate) struct WrappedSubscription(pub Subscription, Private);

#[allow(non_snake_case)]
#[pymethods]
impl WrappedSubscription {
    /// TODO
    #[classattr]
    fn NONE() -> Self {
        NoSubscription.into()
    }

    /// TODO
    #[classattr]
    fn UNCAPTURED() -> Self {
        Uncaptured.into()
    }
}
