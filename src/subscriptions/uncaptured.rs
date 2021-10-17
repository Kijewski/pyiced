use iced::Subscription;
use iced_native::subscription::events;
use pyo3::prelude::*;

use super::ToSubscription;
use crate::app::Interop;
use crate::common::{GCProtocol, Message};

pub(crate) fn init_mod(_py: Python, _m: &PyModule) -> PyResult<()> {
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Uncaptured;

impl GCProtocol for Uncaptured {}

impl ToSubscription for Uncaptured {
    fn to_subscription(&self, _interop: &Interop) -> Subscription<Message> {
        events().map(Message::Native)
    }
}
