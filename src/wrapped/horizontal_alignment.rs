crate::wrap_rust_enum!(
    "HorizontalAlignment" -> WrappedHorizontalAlignment(iced::HorizontalAlignment) {
        LEFT -> iced::HorizontalAlignment::Left,
        CENTER -> iced::HorizontalAlignment::Center,
        RIGHT -> iced::HorizontalAlignment::Right,
    }
);
