crate::wrap_rust_enum!(
    /// The position of the tooltip.
    ///
    /// See also
    /// --------
    /// `iced::widget::tooltip::Position <https://docs.rs/iced/0.3.0/iced/widget/tooltip/enum.Position.html>`_
    "TooltipPosition" -> WrappedTooltipPosition(iced::tooltip::Position) {
        /// The tooltip will follow the cursor.
        FOLLOW_CURSOR -> FollowCursor,
        /// The tooltip will appear on the top of the widget.
        TOP -> Top,
        /// The tooltip will appear on the bottom of the widget.
        BOTTOM -> Bottom,
        /// The tooltip will appear on the left of the widget.
        LEFT -> Left,
        /// The tooltip will appear on the right of the widget.
        RIGHT -> Right,
    }
);
