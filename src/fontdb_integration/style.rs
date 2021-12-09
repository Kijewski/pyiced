crate::wrap_rust_enum!(
    /// Allows italic or oblique faces to be selected.
    ///
    /// See also
    /// --------
    /// `fontdb::Style <https://docs.rs/fontdb/0.7.0/fontdb/enum.Style.html>`_
    "FontStyle" -> WrappedFontStyle(fontdb::Style) {
        /// A face that is neither italic not obliqued.
        NORMAL -> Normal,
        /// A form that is generally cursive in nature.
        ITALIC -> Italic,
        /// A typically-sloped version of the regular face.
        OBLIQUE -> Oblique,
    }
);
