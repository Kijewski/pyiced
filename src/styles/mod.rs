use pyo3::prelude::*;

macro_rules! init_mod {
    ($($module:ident -> $typ:ident),* $(,)?) => {
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
    button -> WrappedButtonStyle,
}
