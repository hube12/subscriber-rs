use futures::channel::mpsc::{channel, Receiver, SendError, Sender};
use futures::SinkExt;
use tokio::task::JoinHandle;

use crate::{
    Subscriber, SubscriberCallback, SubscriberConfig, SubscriberError, SubscriberEvent,
    SubscriberEventType, SubscriberServer,
};

pub struct SubscriberServerHandle<Type, Event, Callback>
where
    Event: SubscriberEvent,
    Type: SubscriberEventType,
    Callback: SubscriberCallback<Event>,
{
    send_event: EventHandle<Event>,
    send_subscribe: SubscribeHandle<Type, Callback>,
    join_handle: JoinHandle<()>,
}

pub struct SubscribeHandle<Type, Callback> {
    send_subscribe: Sender<(Type, Callback)>,
}

pub struct EventHandle<Event> {
    send_event: Sender<Event>,
}

impl<Type, Callback> SubscribeHandle<Type, Callback> {
    pub fn subscribe(
        &mut self,
        event_type: Type,
        callback: Callback,
    ) -> futures::sink::Send<'_, Sender<(Type, Callback)>, (Type, Callback)> {
        self.send_subscribe.send((event_type, callback))
    }
}

impl<Event> EventHandle<Event> {
    pub fn send(&mut self, event: Event) -> futures::sink::Send<'_, Sender<Event>, Event> {
        self.send_event.send(event)
    }
}

impl<Type, Event, Callback> SubscriberServerHandle<Type, Event, Callback>
where
    Event: SubscriberEvent<Type = Type>,
    Type: SubscriberEventType + 'static,
    Callback: SubscriberCallback<Event>,
{
    pub fn new<Config, Sub, Error>(
        server: SubscriberServer<Type, Event, Config, Sub>,
        rt: &tokio::runtime::Handle,
    ) -> Self
    where
        Config: SubscriberConfig + 'static,
        Error: SubscriberError,
        Sub: Subscriber<Event, Error> + 'static,
        Receiver<Callback>: std::marker::Send + 'static,
        Receiver<Event>: std::marker::Send + 'static,
    {
        let (send_event, recv_event) = channel(server.config().channel_size());
        let (send_subscribe, recv_subscribe) = channel(server.config().channel_size());
        let join_handle = rt.spawn(async move { server.run(recv_event, recv_subscribe) });
        Self {
            send_event: EventHandle { send_event },
            send_subscribe: SubscribeHandle { send_subscribe },
            join_handle,
        }
    }

    pub fn split(
        self,
    ) -> (
        EventHandle<Event>,
        SubscribeHandle<Type, Callback>,
        JoinHandle<()>,
    ) {
        (self.send_event, self.send_subscribe, self.join_handle)
    }

    /// This does not stop the process, you need to kill it first
    pub fn stop_handle(&self) {
        self.join_handle.abort();
    }

    pub async fn send(&mut self, event: Event) -> Result<(), SendError> {
        self.send_event.send(event).await
    }

    pub async fn subscribe(
        &mut self,
        event_type: Type,
        callback: Callback,
    ) -> Result<(), SendError> {
        self.send_subscribe.subscribe(event_type, callback).await
    }
}
