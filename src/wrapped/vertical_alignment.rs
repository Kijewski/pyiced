crate::wrap_rust_enum!(
    "VerticalAlignment" -> WrappedVerticalAlignment(iced::VerticalAlignment) {
        TOP -> iced::VerticalAlignment::Top,
        CENTER -> iced::VerticalAlignment::Center,
        BOTTOM -> iced::VerticalAlignment::Bottom,
    }
);
