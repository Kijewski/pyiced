// Copyright (c) 2021-2022 René Kijewski <pypi.org@k6i.de>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License

#![allow(clippy::too_many_arguments)]

use mimalloc::MiMalloc;
use pyo3::prelude::*;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

macro_rules! init_mod {
    ($(mod $name:ident $($feature:literal)?;)*) => {
        $(
            $( #[cfg(feature = $feature)] )?
            mod $name;
        )*

        #[pymodule]
        fn init_mod(py: Python, m: &PyModule) -> PyResult<()> {
            $(
                $( #[cfg(feature = $feature)] )?
                {
                    $name::init_mod(py, m)?;
                }
            )*
            Ok(())
        }
    };
}

init_mod! {
    mod app;
    mod async_tasks;
    mod common;
    mod extractor;
    mod fontdb_integration "fontdb";
    mod states;
    mod styles;
    mod subscriptions;
    mod widgets;
    mod wrapped;
}

#[pymodule]
fn _pyiced(py: Python, m: &PyModule) -> PyResult<()> {
    init_mod(py, m)?;
    m.add("__version__", env!("pyiced-version"))?;
    m.add("__author__", env!("CARGO_PKG_AUTHORS"))?;
    m.add("__license__", env!("CARGO_PKG_LICENSE"))?;
    Ok(())
}

#[macro_export]
macro_rules! assign {
    ($input:expr, $self:ident, $($name:ident),* $(,)?) => {
        {
            let el = $input;
            let this = $self;
            $(
                let el = match this.$name {
                    ::std::option::Option::Some(value) => el.$name(value),
                    ::std::option::Option::None => el,
                };
            )*
            el
        }
    };
}

#[macro_export]
macro_rules! make_with_state {
    ($name:ident($WidgetWoLifetime:ty, $Widget:ty, $State:ty $(,)?) $(,)?;) => {
        mod __mod_with_state {
            use std::ops::DerefMut;
            use std::sync::Arc;

            use iced::{Element, Length, Point, Rectangle};
            use iced_native::layout::{Limits, Node};
            use iced_native::Widget;
            #[cfg(feature = "wgpu")]
            use iced_wgpu::Renderer;
            use ouroboros::self_referencing;
            use parking_lot::lock_api::ArcRwLockWriteGuard;
            use parking_lot::{RawRwLock, RwLock};
            use $crate::common::{empty_space, Message};

            #[self_referencing]
            struct WidgetWithState {
                _guard: ArcRwLockWriteGuard<RawRwLock, $State>,
                #[borrows(mut _guard)]
                #[covariant]
                widget: $Widget,
            }

            impl Widget<Message, Renderer> for WidgetWithState {
                fn width(&self) -> Length {
                    self.with_widget(|widget| {
                        <$WidgetWoLifetime as Widget<Message, Renderer>>::width(widget)
                    })
                }

                fn height(&self) -> Length {
                    self.with_widget(|widget| {
                        <$WidgetWoLifetime as Widget<Message, Renderer>>::height(widget)
                    })
                }

                fn layout(&self, renderer: &Renderer, limits: &Limits) -> Node {
                    self.with_widget(|widget| {
                        <$WidgetWoLifetime as Widget<Message, Renderer>>::layout(
                            widget, renderer, limits,
                        )
                    })
                }

                fn draw(
                    &self,
                    renderer: &mut Renderer,
                    defaults: &<Renderer as iced_native::Renderer>::Defaults,
                    layout: iced_native::Layout<'_>,
                    cursor_position: Point,
                    viewport: &Rectangle,
                ) -> <Renderer as iced_native::Renderer>::Output {
                    self.with_widget(|widget| {
                        <$WidgetWoLifetime as Widget<Message, Renderer>>::draw(
                            widget,
                            renderer,
                            defaults,
                            layout,
                            cursor_position,
                            viewport,
                        )
                    })
                }

                fn hash_layout(&self, state: &mut iced_native::Hasher) {
                    self.with_widget(|widget| {
                        <$WidgetWoLifetime as Widget<Message, Renderer>>::hash_layout(widget, state)
                    })
                }

                fn on_event(
                    &mut self,
                    event: iced_native::Event,
                    layout: iced_native::Layout<'_>,
                    cursor_position: Point,
                    renderer: &Renderer,
                    clipboard: &mut dyn iced_native::Clipboard,
                    messages: &mut Vec<Message>,
                ) -> iced_native::event::Status {
                    self.with_widget_mut(|widget| {
                        <$WidgetWoLifetime as Widget<Message, Renderer>>::on_event(
                            widget,
                            event,
                            layout,
                            cursor_position,
                            renderer,
                            clipboard,
                            messages,
                        )
                    })
                }

                fn overlay(
                    &mut self,
                    layout: iced_native::Layout<'_>,
                ) -> Option<iced_native::overlay::Element<'_, Message, Renderer>> {
                    // TODO: is this transmute sound?
                    let widget: &mut $WidgetWoLifetime =
                        self.with_widget_mut(|widget| unsafe { std::mem::transmute(widget) });
                    widget.overlay(layout)
                }
            }

            #[allow(dead_code)]
            pub(crate) fn with_state(
                arc: &Arc<RwLock<$State>>,
                make: impl for<'this> FnOnce(&'this mut $State) -> Result<$Widget, ()>,
            ) -> Element<'static, Message> {
                let guard = match arc.try_write_arc() {
                    Some(guard) => guard,
                    None => return empty_space(),
                };
                let builder = WidgetWithStateTryBuilder {
                    _guard: guard,
                    widget_builder: move |guard| make(guard.deref_mut()),
                };
                match builder.try_build() {
                    Ok(widget) => Element::new(widget),
                    Err(_) => empty_space(),
                }
            }
        }

        pub(crate) use __mod_with_state::with_state as $name;
    };
}

#[macro_export]
macro_rules! wrap_rust_enum {
    (
        $( #[doc = $class_doc:expr] )+
        $Name:literal -> $WrappedName:ident($RustType:ty)
        {
            $(
                $( #[doc = $value_doc:expr] )+
                $UpperCase:ident -> $Value:ident
            ),*
            $(,)?
        }
    ) => {
        use pyo3::prelude::*;
        use $crate::common::debug_str;

        pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
            m.add_class::<$WrappedName>()?;
            Ok(())
        }

        $( #[doc = $class_doc] )*
        #[doc = ""]
        #[doc = "Attributes"]
        #[doc = "----------"]
        $(
            #[doc = stringify!($UpperCase)]
            $( #[doc = concat!("    ", $value_doc)] )*
        )*
        #[pyclass(
            name = $Name,
            module = "pyiced",
            freelist = $crate::CountIdents!($($UpperCase)*),
        )]
        #[derive(Debug, Clone)]
        pub(crate) struct $WrappedName(pub $RustType);

        #[pymethods]
        impl $WrappedName {
            $(
                $( #[doc = $value_doc] )*
                #[classattr]
                #[allow(non_snake_case)]
                fn $UpperCase() -> Self {
                    Self(<$RustType>::$Value)
                }
            )*

            fn __str__(&self) -> PyResult<String> {
                debug_str(&self.0)
            }

            fn __repr__(&self) -> &'static str {
                match self.0 {
                    $(
                        <$RustType>::$Value => concat!($Name, ".", stringify!($UpperCase))
                    ),*
                }
            }
        }
    };
}

#[macro_export]
macro_rules! CountIdents {
    ($($idents:ident)*) => {
        $( $crate::CountIdents!(> $idents) + )* 0
    };
    (>$ident:ident) => ( 1 );
}

#[macro_export]
macro_rules! extract_multiple {
    ($kwargs:ident, $init:expr, $($name:ident),* $(,)?) => {
        {
            let mut result = $init;
            let kwargs = match $kwargs {
                Some(kwargs) => kwargs,
                None => return Ok(Self(result)),
            };
            for (key, value) in kwargs.iter() {
                let key = key.str()?;
                let value = $crate::extractor::Extractor(value);
                match key.to_str()? {
                    $(
                        stringify!($name) => result.0.$name = value.try_into()?,
                    )*
                    key => return Err(PyErr::new::<PyValueError, _>(
                        $crate::format_to_string_ignore!("Unknown keyword argument: {:#?}", key)
                    )),
                }
            }
            return Ok(Self(result));
        }
    };
}

#[macro_export]
macro_rules! dyn_style_proto_get {
    ( $proto:expr, $get:expr, $default:ident $(, $alt:ident)* $(,)? ) => {
        match $proto {
            Some(proto) => match proto.extract() {
                Ok(Self(proto)) => proto.0,
                Err(_) => match proto.downcast::<PyString>() {
                    Ok(s) => match s.to_str()? {
                        stringify!($default) => $get(Box::<dyn StyleSheet>::default().$default()),
                        $( stringify!($alt) => $get(Box::<dyn StyleSheet>::default().$alt()), )*
                        s => return Err(PyErr::new::<PyValueError, _>(
                            $crate::format_to_string_ignore!("Unknown proto value: {:#}", s),
                        )),
                    },
                    Err(err) => {
                        return Err($crate::common::debug_err::<PyValueError, _>(err));
                    }
                }
            },
            None => $get(Box::<dyn StyleSheet>::default().$default()),
        }
    };
}

#[macro_export]
macro_rules! dyn_style_proto {
    ( $proto:expr, $default:ident $(, $alt:ident)* $(,)? ) => {
        $crate::dyn_style_proto_get!($proto, |o| o, $default, $($alt),*)
    };
}

#[macro_export]
macro_rules! format_to_string {
    ($($arg:tt)+) => {
        {
            use std::fmt::Write as _;

            type Result = ::std::result::Result::<
                ::std::string::String,
                &'static ::std::primitive::str,
            >;

            let mut s = ::std::string::String::new();
            match ::std::write!(s, $($arg)+) {
                ::std::result::Result::Ok(_) => Result::Ok(s),
                ::std::result::Result::Err(_) => Result::Err("Could not format"),
            }
        }
    };
}

#[macro_export]
macro_rules! format_to_string_ignore {
    ($($arg:tt)+) => (
        match $crate::format_to_string!($($arg)+) {
            ::std::result::Result::Ok(s) => ::std::borrow::Cow::Owned(s),
            ::std::result::Result::Err(s) => ::std::borrow::Cow::Borrowed(s),
        }
    );
}

#[macro_export]
macro_rules! format_to_py {
    ($error_type:ty, $($arg:tt)+) => (
        $crate::format_to_string!($($arg)+).map_err(|err| {
            ::pyo3::PyErr::new::<$error_type, _>(err)
        })
    );
    ($($arg:tt)+) => (
        $crate::format_to_py!(::pyo3::exceptions::PyRuntimeError, $($arg)+)
    );
}

#[macro_export]
macro_rules! format_to_cow {
    ($($arg:tt)+) => (
        $crate::format_to_py!($($arg)+).map(::std::borrow::Cow::Owned)
    );
}

#[macro_export]
macro_rules! getters {
    (
        $wrapped:ty => $get:expr,
        $( $name:ident -> $python_type:literal $wrapped_type:ty ),* $(,)?
    ) => {
        #[pymethods]
        impl $wrapped {
            $(
                #[doc = concat!(
                    "The \"", stringify!($name), "\" parameter given to the constructor.\n",
                    "\n",
                    "Returns\n",
                    "-------\n",
                    $python_type, "\n",
                    "    The set, copied or defaulted value."
                )]
                #[getter]
                #[allow(clippy::redundant_closure_call)]
                fn $name(&self) -> $wrapped_type {
                    $crate::extractor::Unextract::unextract(
                        $crate::extractor::Unextractor(&($get)(self).$name),
                    )
                }
            )*
        }
    };
}

#[macro_export]
macro_rules! partially_defaulted_stylesheet {
    ($style:ty, $stylesheet:ty $(, $active:ident)* => $param:ident $(,)?) => {
        let $param = match $param {
            ::std::option::Option::Some(o) => o.0.0,
            ::std::option::Option::None => {
                struct Partial<'a> {
                    $( $active: &'a $style),*
                }

                impl $stylesheet for Partial<'_> {
                    $(
                        fn $active(&self) -> Style {
                            *self.$active
                        }
                    )*
                }

                Partial { $($active: &$active),* }.$param()
            },
        };
    };
}
