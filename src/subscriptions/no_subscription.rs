use iced::Subscription;

use super::ToSubscription;
use crate::common::{GCProtocol, Message};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct NoSubscription;

impl GCProtocol for NoSubscription {}

impl ToSubscription for NoSubscription {
    fn to_subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }
}
