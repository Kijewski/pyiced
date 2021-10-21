crate::wrap_rust_enum!(
    "TooltipPosition" -> WrappedTooltipPosition(iced::tooltip::Position) {
        FOLLOW_CURSOR -> iced::tooltip::Position::FollowCursor,
        TOP -> iced::tooltip::Position::Top,
        BOTTOM -> iced::tooltip::Position::Bottom,
        LEFT -> iced::tooltip::Position::Left,
        RIGHT -> iced::tooltip::Position::Right,
    }
);
