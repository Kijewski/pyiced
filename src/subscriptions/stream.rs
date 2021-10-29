use std::hash::Hasher;

use futures_util::stream::BoxStream;
use iced::Subscription;
use iced_native::subscription::Recipe;
use pyo3::prelude::*;

use super::ToSubscription;
use crate::common::{GCProtocol, Message};

pub(crate) fn init_mod(_py: Python, _m: &PyModule) -> PyResult<()> {
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Stream {}

impl GCProtocol for Stream {}

impl ToSubscription for Stream {
    fn to_subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }
}

impl<H, E> Recipe<H, E> for Stream
where
    H: Hasher,
{
    type Output;

    fn hash(&self, state: &mut H) {
        todo!()
    }

    fn stream(self: Box<Self>, input: BoxStream<Message>) -> BoxStream<Self::Output> {
        todo!()
    }
}
