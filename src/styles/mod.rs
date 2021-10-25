use pyo3::prelude::*;

macro_rules! init_mod {
    ($($module:ident -> { $($types:ident),* }),* $(,)?) => {
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
    button -> { ButtonStyle, WrappedButtonStyle },
    // checkbox -> pedCheckboxStyle, WrappedCheckboxStyle },
    container -> { ContainerStyle, WrappedContainerStyle },
    pane_grid -> { PaneGridStyle, WrappedPaneGridStyle },
    // pick_list -> { PickListStyle, WrappedPickListStyle },
    progress_bar -> { ProgressBarStyle, WrappedProgressBarStyle },
    // radio -> { RadioStyle, WrappedRadioStyle },
    // rule -> { RuleStyle, WrappedRuleStyle },
    // scrollable -> { ScrollableStyle, WrappedScrollableStyle },
    // slider -> { SliderStyle, WrappedSliderStyle },
    // text_input -> { TextInputStyle, WrappedTextInputStyle },
}
