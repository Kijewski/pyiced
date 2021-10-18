use pyo3::prelude::*;

macro_rules! init_mod {
    ($(mod $name:ident;)*) => {
        $( mod $name; )*

        #[pymodule]
        fn pyiced(py: Python, m: &PyModule) -> PyResult<()> {
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
            struct WithState {
                _guard: ArcMutexGuard<RawMutex, $State>,
                #[borrows(mut _guard)]
                #[not_covariant]
                widget: $Widget,
            }

            impl<'this> Widget<Message, Renderer> for WithState {
                fn width(&self) -> Length {
                    self.with_widget(|w| <$WidgetWoLifetime as Widget<Message, Renderer>>::width(w))
                }

                fn height(&self) -> Length {
                    self.with_widget(|w| {
                        <$WidgetWoLifetime as Widget<Message, Renderer>>::height(w)
                    })
                }

                fn layout(&self, renderer: &Renderer, limits: &Limits) -> Node {
                    self.with_widget(|w| {
                        <$WidgetWoLifetime as Widget<Message, Renderer>>::layout(
                            w, renderer, limits,
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
                    self.with_widget(|w| {
                        <$WidgetWoLifetime as Widget<Message, Renderer>>::draw(
                            w,
                            renderer,
                            defaults,
                            layout,
                            cursor_position,
                            viewport,
                        )
                    })
                }

                fn hash_layout(&self, state: &mut iced_native::Hasher) {
                    self.with_widget(|w| {
                        <$WidgetWoLifetime as Widget<Message, Renderer>>::hash_layout(w, state)
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
                    self.with_widget_mut(|w| {
                        <$WidgetWoLifetime as Widget<Message, Renderer>>::on_event(
                            w,
                            event,
                            layout,
                            cursor_position,
                            renderer,
                            clipboard,
                            messages,
                        )
                    })
                }

                // TODO:
                // fn overlay(
                //     &mut self,
                //     layout: iced_native::Layout<'_>,
                // ) -> Option<iced_native::overlay::Element<'_, Message, Renderer>> {
                //     self.with_widget_mut(|w| {
                //         <$WidgetWoLifetime as Widget<Message, Renderer>>::overlay(
                //             w, layout,
                //         )
                //     })
                // }
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
                let builder = WithStateTryBuilder {
                    _guard: guard,
                    widget_builder: move |guard| make(guard.deref_mut()),
                };
                match builder.try_build() {
                    Ok(w) => Element::new(w),
                    Err(_) => empty_space(),
                }
            }
        }

        pub(crate) use __mod_with_state::with_state as $name;
    };
}
