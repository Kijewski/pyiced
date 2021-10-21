#![allow(clippy::too_many_arguments)]

use mimalloc::MiMalloc;
use pyo3::prelude::*;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

macro_rules! init_mod {
    ($(mod $name:ident;)*) => {
        $( mod $name; )*

        #[pymodule]
        fn init_mod(py: Python, m: &PyModule) -> PyResult<()> {
            $( $name::init_mod(py, m)?; )*
            Ok(())
        }
    };
}

init_mod! {
    mod app;
    mod common;
    mod states;
    mod widgets;
    mod wrapped;
}

#[pymodule]
fn pyiced(py: Python, m: &PyModule) -> PyResult<()> {
    init_mod(py, m)?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add("__author__", env!("CARGO_PKG_AUTHORS"))?;
    Ok(())
}

#[macro_export]
macro_rules! assign {
    ($input:expr, $self:ident, $name:ident $(,)?) => {
        match ($input, $self.$name) {
            (input, ::std::option::Option::Some(value)) => input.$name(value),
            (input, ::std::option::Option::None) => input,
        }
    };
    ($input:expr, $self:ident, $name:ident, $($names:ident),+ $(,)?) => {
        assign!(
            assign!($input, $self, $name),
            $self, $($names),+
        )
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
            use iced_native::{self, Widget};
            use iced_wgpu::Renderer;
            use ouroboros::self_referencing;
            use parking_lot::lock_api::ArcMutexGuard;
            use parking_lot::{Mutex, RawMutex};

            use crate::common::{empty_space, Message};

            #[self_referencing]
            struct WidgetWithState {
                _guard: ArcMutexGuard<RawMutex, $State>,
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
                arc: Option<&Arc<Mutex<$State>>>,
                make: impl for<'this> FnOnce(&'this mut $State) -> Result<$Widget, ()>,
            ) -> Element<'static, Message> {
                let guard = match arc.and_then(|arc| arc.try_lock_arc()) {
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
        $Name:literal -> $WrappedName:ident($RustType:ty)
        {
            $($UpperCase:ident -> $Value:expr),* $(,)?
        }
    ) => {
        use pyo3::prelude::*;
        use pyo3::PyObjectProtocol;

        use crate::common::debug_str;

        pub(crate) fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
            m.add_class::<$WrappedName>()?;
            Ok(())
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        struct Private;

        #[pyclass(
            name = $Name,
            module = "pyiced.pyiced",
            freelist = $crate::CountIdents!($($UpperCase)*),
        )]
        #[derive(Debug, Clone)]
        pub(crate) struct $WrappedName(pub $RustType, Private);

        #[pymethods]
        impl $WrappedName {
            $(
                #[classattr]
                #[allow(non_snake_case)]
                fn $UpperCase() -> Self {
                    Self($Value, Private)
                }
            )*
        }

        #[pyproto]
        impl PyObjectProtocol for $WrappedName {
            fn __str__(&self) -> PyResult<String> {
                debug_str(&self.0)
            }
        }
    };
}

#[macro_export]
macro_rules! CountIdents {
    () => {
        0
    };
    ($ident:ident $($idents:ident)*) => {
        (1 + $crate::CountIdents!($($idents)*))
    };
}
