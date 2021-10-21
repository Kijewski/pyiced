crate::wrap_rust_enum!(
    "Align" -> WrappedAlign(iced::Align) {
        START -> iced::Align::Start,
        CENTER -> iced::Align::Center,
        END -> iced::Align::End,
    }
);
