crate::wrap_rust_enum!(
    /// The horizontal alignment of some resource.
    ///
    /// See also
    /// --------
    /// `iced::HorizontalAlignment <https://docs.rs/iced/0.3.0/iced/enum.HorizontalAlignment.html>`_
    "HorizontalAlignment" -> WrappedHorizontalAlignment(iced::HorizontalAlignment) {
        /// Align left
        LEFT -> Left,
        /// Horizontally centered
        CENTER -> Center,
        /// Align right
        RIGHT -> Right,
    }
);
