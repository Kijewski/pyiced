crate::wrap_rust_enum!(
    /// Alignment on an axis of a container.
    ///
    /// See also
    /// --------
    /// `iced::Align <https://docs.rs/iced/0.3.0/iced/enum.Align.html>`_
    "Align" -> WrappedAlign(iced::Align) {
        /// Align at the start of the axis.
        START -> Start,
        /// Align at the center of the axis.
        CENTER -> Center,
        /// Align at the end of the axis.
        END -> End,
    }
);
