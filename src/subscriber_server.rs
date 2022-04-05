use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;

use crate::subscriber::Subscriber;
use crate::subscriber_config::SubscriberConfig;
use crate::subscribers::Subscribers;

#[derive(Debug)]
pub struct SubscriberServer<Type, Event, Config, Sub> {
    store: HashMap<Type, VecDeque<Event>>,
    config: Config,
    subscribers: HashMap<Type, Subscribers<Sub>>,
}

impl<Type, Event, Config, Sub> Default for SubscriberServer<Type, Event, Config, Sub>
where
    Config: SubscriberConfig,
{
    fn default() -> Self {
        SubscriberServer::new(Config::default())
    }
}

impl<Type, Event, Config, Sub> SubscriberServer<Type, Event, Config, Sub> {
    pub fn new(config: Config) -> Self {
        Self {
            store: HashMap::with_capacity(16 + 2 + 1 + 1),
            config,
            subscribers: HashMap::with_capacity(10),
        }
    }

    pub fn run(self) {
        loop {
            // Do subscription
        }
    }
}

impl<Type, Event: Clone, Config: SubscriberConfig, Sub> SubscriberServer<Type, Event, Config, Sub> {
    pub fn send<Error: std::error::Error>(&mut self, event_type: Type, event: Event)
    where
        Event: Clone,
        Sub: Subscriber<Event, Error>,
        Type: Hash + Eq,
    {
        if let Some(subscribers) = self.subscribers.get(&event_type) {
            subscribers.notify(event);
        } else {
            self.store
                .entry(event_type)
                .or_insert(VecDeque::with_capacity(self.config.channel_size()))
                .push_back(event)
        }
    }

    pub fn subscribe<
        T: 'static + Fn(Event) + std::panic::UnwindSafe + std::panic::RefUnwindSafe,
        Error: std::error::Error,
    >(
        &mut self,
        event: Type,
        callback: T,
    ) -> Result<(), Error>
    where
        Sub: Subscriber<Event, Error>,
        Type: Hash + Eq,
    {
        self.subscribers
            .entry(event)
            .or_insert(Subscribers::new())
            .push(Subscriber::new(callback));
        Ok(())
    }
}
