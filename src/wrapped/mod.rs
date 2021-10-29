use pyo3::prelude::*;

macro_rules! init_mod {
    ($($module:ident -> $typ:ident),+ $(,)?) => {
        $( mod $module; )*

        #[allow(unused_imports)]
        pub(crate) use {
            $( $module :: $typ ),*
        };

        pub(crate) fn init_mod(py: Python, m: &PyModule) -> PyResult<()> {
            $( $module::init_mod(py, m)?; )*
            Ok(())
        }
    };
}

init_mod! {
    align -> WrappedAlign,
    color -> WrappedColor,
    font -> WrappedFont,
    horizontal_alignment -> WrappedHorizontalAlignment,
    image -> WrappedImageHandle,
    instant -> WrappedInstant,
    length -> WrappedLength,
    line -> WrappedLine,
    message -> WrappedMessage,
    slider_handle -> WrappedSliderHandle,
    slider_handle_shape -> WrappedSliderHandleShape,
    svg -> WrappedSvgHandle,
    tooltip_position -> WrappedTooltipPosition,
    vertical_alignment -> WrappedVerticalAlignment,
}
