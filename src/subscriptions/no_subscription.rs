use iced::Subscription;
use pyo3::prelude::*;

use super::ToSubscription;
use crate::app::Interop;
use crate::common::{GCProtocol, Message};

pub(crate) fn init_mod(_py: Python, _m: &PyModule) -> PyResult<()> {
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct NoSubscription;

impl GCProtocol for NoSubscription {}

impl ToSubscription for NoSubscription {
    fn to_subscription(&self, _interop: &Interop) -> Subscription<Message> {
        Subscription::none()
    }
}
