use alloc::vec;
use alloc::vec::Vec;

use crate::{Subscriber, SubscriberError, SubscriberEvent};

#[derive(Debug)]
pub struct Subscribers<Sub> {
    subscribers: Vec<Sub>,
}

impl<Sub> Subscribers<Sub> {
    pub fn new(sub_count: usize) -> Self {
        Self {
            subscribers: Vec::with_capacity(sub_count),
        }
    }

    pub fn push<Event, Error>(&mut self, subscriber: Sub)
    where
        Event: SubscriberEvent,
        Error: SubscriberError,
        Sub: Subscriber<Event, Error>,
    {
        self.subscribers.push(subscriber);
    }

    pub fn notify<Event, Error>(&self, event: Event) -> Option<Vec<Error>>
    where
        Event: SubscriberEvent,
        Error: SubscriberError,
        Sub: Subscriber<Event, Error>,
    {
        if self.subscribers.len() == 1 {
            match self.subscribers.first().unwrap().notify(event) {
                Ok(..) => None,
                Err(e) => Some(vec![e]),
            }
        } else {
            let mut errors = Vec::with_capacity(self.subscribers.len());
            for sub in &self.subscribers {
                if let Err(e) = sub.notify(event.clone()) {
                    errors.push(e)
                }
            }
            if errors.len() > 0 {
                Some(errors)
            } else {
                None
            }
        }
    }
}
