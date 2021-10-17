crate::wrap_rust_enum!(
    /// The vertical alignment of some resource.
    ///
    /// See also
    /// --------
    /// `iced::VerticalAlignment <https://docs.rs/iced/0.3.0/iced/enum.VerticalAlignment.html>`_
    "VerticalAlignment" -> WrappedVerticalAlignment(iced::VerticalAlignment) {
        /// Align top
        TOP -> Top,
        /// Vertically centered
        CENTER -> Center,
        /// Align bottom
        BOTTOM -> Bottom,
    }
);
