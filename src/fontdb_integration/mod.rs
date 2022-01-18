use pyo3::prelude::*;

macro_rules! init_mod {
    ($($module:ident -> { $($types:ident),* $(,)? }),* $(,)?) => {
        $( mod $module; )*

        #[allow(unused_imports)]
        pub(crate) use self::{ $( $module :: { $($types),* } ),* };

        pub(crate) fn init_mod(py: Python, m: &PyModule) -> PyResult<()> {
            $( $module::init_mod(py, m)?; )*
            Ok(())
        }
    };
}

init_mod! {
    family -> { WrappedFontFamily },
    id -> { WrappedFontId },
    stretch -> { WrappedFontStretch },
    style -> { WrappedFontStyle },
    systemfonts -> { },
    weight -> { WrappedFontWeight },
}
