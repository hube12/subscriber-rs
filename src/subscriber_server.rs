use alloc::collections::{BTreeMap, VecDeque};
use core::borrow::BorrowMut;

use futures::channel::mpsc::Receiver;

use crate::{
    Subscriber, SubscriberCallback, SubscriberConfig, SubscriberError, SubscriberEvent,
    SubscriberEventType, Subscribers,
};

pub struct SubscriberServer<Type, Event, Config, Sub> {
    store: BTreeMap<Type, VecDeque<Event>>,
    config: Config,
    subscribers: BTreeMap<Type, crate::Subscribers<Sub>>,
}

impl<Type, Event, Config, Sub> Default for SubscriberServer<Type, Event, Config, Sub>
where
    Config: SubscriberConfig,
{
    fn default() -> Self {
        SubscriberServer::new(Config::default())
    }
}

impl<Type, Event, Config, Sub> SubscriberServer<Type, Event, Config, Sub>
where
    Config: SubscriberConfig,
{
    pub fn new(config: Config) -> Self {
        Self {
            store: BTreeMap::new(),
            config,
            subscribers: BTreeMap::new(),
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}

impl<Type, Event, Config, Sub> SubscriberServer<Type, Event, Config, Sub>
where
    Event: SubscriberEvent<Type = Type>,
    Config: SubscriberConfig,
    Type: SubscriberEventType,
{
    pub fn run<Error, Callback>(
        mut self,
        mut recv_event: Receiver<Event>,
        mut recv_subscribe: Receiver<(Type, Callback)>,
    ) where
        Error: SubscriberError,
        Sub: Subscriber<Event, Error>,
        Callback: SubscriberCallback<Event> + 'static,
    {
        loop {
            // Do subscription
            while let Ok(msg) = recv_subscribe.try_next() {
                match msg {
                    None => {
                        log::warn!("Channel is closed");
                        break;
                    }
                    Some((event_type, callback)) => {
                        if let Err(e) = self.borrow_mut().subscribe(event_type, callback) {
                            log::warn!(
                                "Could not subscribe, this can not be recovered yet, {:?}",
                                e
                            );
                        }
                    }
                }
            }
            while let Ok(msg) = recv_event.try_next() {
                match msg {
                    None => {
                        log::warn!("Channel is closed");
                        break;
                    }
                    Some(event) => {
                        if event.should_kill() {
                            return;
                        }
                        self.borrow_mut().send(event);
                    }
                }
            }
        }
    }
}

impl<Type, Event, Config, Sub> SubscriberServer<Type, Event, Config, Sub>
where
    Type: SubscriberEventType,
    Event: SubscriberEvent<Type = Type>,
    Config: SubscriberConfig,
{
    pub fn send<Error>(&mut self, event: Event)
    where
        Error: SubscriberError,
        Sub: Subscriber<Event, Error>,
    {
        let event_type = event.get_type();
        if let Some(subscribers) = self.subscribers.get(&event_type) {
            subscribers.notify(event);
        } else {
            self.store
                .entry(event_type)
                .or_insert(VecDeque::with_capacity(self.config.channel_size()))
                .push_back(event)
        }
    }
}

impl<Type, Event, Config, Sub> SubscriberServer<Type, Event, Config, Sub>
where
    Type: SubscriberEventType,
    Event: SubscriberEvent,
    Config: SubscriberConfig,
{
    pub fn subscribe<Callback, Error>(
        &mut self,
        event_type: Type,
        callback: Callback,
    ) -> Result<(), Error>
    where
        Error: SubscriberError,
        Sub: Subscriber<Event, Error>,
        Callback: SubscriberCallback<Event> + 'static,
    {
        self.subscribers
            .entry(event_type)
            .or_insert(Subscribers::new(self.config.subscriber_count()))
            .push(Subscriber::new(callback));
        Ok(())
    }
}
