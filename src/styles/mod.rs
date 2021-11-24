use pyo3::prelude::*;

macro_rules! init_mod {
    ($($module:ident -> { $($types:ident),* $(,)? }),* $(,)?) => {
        $( mod $module; )*

        #[allow(unused_imports)]
        pub(crate) use {
            $( $module :: { $($types),* } ),*
        };

        pub(crate) fn init_mod(py: Python, m: &PyModule) -> PyResult<()> {
            $( $module::init_mod(py, m)?; )*
            Ok(())
        }
    };
}

init_mod! {
    button -> { ButtonStyle, WrappedButtonStyleSheet },
    checkbox -> {
        CheckboxStyle, CheckboxStyleSheet, WrappedCheckboxStyle, WrappedCheckboxStyleSheet,
    },
    container -> { ContainerStyle, WrappedContainerStyle },
    pane_grid -> { PaneGridStyle, WrappedPaneGridStyle },
    pick_list -> {
        PickListMenu, PickListStyle, PickListStyleSheet, WrappedPickListMenu, WrappedPickListStyle,
        WrappedPickListStyleSheet,
    },
    progress_bar -> { ProgressBarStyle, WrappedProgressBarStyle },
    radio -> {
        RadioStyle, RadioStyleSheet, WrappedRadioStyle, WrappedRadioStyleSheet,
    },
    rule -> { RuleStyle, WrappedRuleStyleSheet },
    scrollable -> {
        ScrollableStyleSheet, ScrollbarStyle, ScrollerStyle, WrappedScrollableStyleSheet,
        WrappedScrollbarStyle,WrappedScrollerStyle,
    },
    slider -> { SliderStyle, SliderStyleSheet, WrappedSliderStyle, WrappedSliderStyleSheet },
    text_input -> {
        TextInputStyle, TextInputStyleSheet, WrappedTextInputStyle, WrappedTextInputStyleSheet,
    },
}
