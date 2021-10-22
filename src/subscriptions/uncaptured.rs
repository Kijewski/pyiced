use iced::Subscription;
use iced_native::subscription::events;

use super::ToSubscription;
use crate::common::{GCProtocol, Message};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Uncaptured;

impl GCProtocol for Uncaptured {}

impl ToSubscription for Uncaptured {
    fn to_subscription(&self) -> Subscription<Message> {
        events().map(Message::Native)
    }
}
