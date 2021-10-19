#![allow(unused_imports)]

use pyo3::prelude::*;

macro_rules! init_mod {
    ($($module:ident -> { $($typ:ident),* }),+ $(,)?) => {
        $( mod $module; )*

        $( pub(crate) use $module::{$($typ),*}; )*

        pub(crate) fn init_mod(py: Python, m: &PyModule) -> PyResult<()> {
            $( $module::init_mod(py, m)?; )*
            Ok(())
        }
    };
}

init_mod! {
    button_state -> { ButtonState, WrappedButtonState, button_with_state },
    pick_list_state -> { PickListState, WrappedPickListState, pick_list_with_state },
    scrollable_state -> { ScrollableState, WrappedScrollableState, scrollable_with_state },
    slider_state -> { SliderState, WrappedSliderState, slider_with_state },
    text_input_state -> { TextInputState, WrappedTextInputState, text_input_with_state },
}
