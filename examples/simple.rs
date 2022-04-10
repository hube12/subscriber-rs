use std::fmt::{Debug, Display, Formatter};
use std::panic::{catch_unwind, UnwindSafe};
use std::time::Duration;

use anyhow::anyhow;

use subscriber_rs::{
    SubscribeHandle, Subscriber, SubscriberCallback, SubscriberError, SubscriberEvent,
    SubscriberEventType, SubscriberServer, SubscriberServerHandle,
};

#[tokio::main]
async fn main() {
    let mut server: SubscriberServer<SEventType, SEvent, SubscriberConfig, Sub<SEvent>> =
        SubscriberServer::new(SubscriberConfig::default());
    let mut handle = SubscriberServerHandle::new(server, &tokio::runtime::Handle::current());
    let f: fn(SEvent) = |e| println!("FROM CALLBACK : {:?}", e);
    let _ = dbg!(handle.subscribe(SEventType::ErrorParsing, f).await);
    let _ = dbg!(
        handle
            .send(SEvent::ErrorParsing(String::from("test")))
            .await
    );
    let _ = dbg!(handle.send(SEvent::Kill).await);
    let _ = dbg!(handle.stop_handle());
    let _ = dbg!(
        handle
            .send(SEvent::ErrorParsing(String::from("should fail")))
            .await
    );
    let _ = dbg!(
        handle
            .send(SEvent::ErrorParsing(String::from("should fail")))
            .await
    );
    let _ = dbg!(
        handle
            .send(SEvent::ErrorParsing(String::from("should fail")))
            .await
    );
}

#[derive(Clone, Debug)]
pub enum SEvent {
    ErrorIO((u32, String)),
    ErrorParsing(String),
    Event((u16, String)),
    Kill,
}
impl UnwindSafe for SEvent {}

impl SubscriberCallback<SEvent> for fn(SEvent) {}

impl SubscriberEvent for SEvent {
    type Type = SEventType;

    fn should_kill(&self) -> bool {
        matches!(self.get_type(), Self::Type::Kill)
    }

    fn get_type(&self) -> Self::Type {
        match self {
            SEvent::ErrorIO((code, _)) => Self::Type::ErrorIO(*code),
            SEvent::ErrorParsing(_) => Self::Type::ErrorParsing,
            SEvent::Event((event_type, _)) => Self::Type::Event(*event_type),
            SEvent::Kill => Self::Type::Kill,
        }
    }
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SEventType {
    ErrorIO(u32),
    Event(u16),
    ErrorParsing,
    Kill,
}
impl SubscriberEventType for SEventType {}

#[derive(Debug)]
pub struct SubscriberConfig {
    channel_size: u32,
    subscriber_count: u32,
}

impl subscriber_rs::SubscriberConfig for SubscriberConfig {
    fn subscriber_count(&self) -> usize {
        self.subscriber_count as usize
    }

    fn channel_size(&self) -> usize {
        self.channel_size as usize
    }
}

impl Default for SubscriberConfig {
    fn default() -> Self {
        Self {
            channel_size: 1024,
            subscriber_count: 3,
        }
    }
}

pub struct Sub<Event> {
    callback: Box<dyn SubscriberCallback<Event>>,
}

#[derive(Debug)]
pub struct SError {
    error: anyhow::Error,
}

impl SubscriberError for SError {}

impl From<anyhow::Error> for SError {
    fn from(error: anyhow::Error) -> Self {
        Self { error }
    }
}

impl Display for SError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl std::error::Error for SError {}

impl Subscriber<SEvent, SError> for Sub<SEvent> {
    fn new<T: SubscriberCallback<SEvent> + 'static>(callback: T) -> Self {
        Self {
            callback: Box::new(callback),
        }
    }

    fn notify(&self, event: SEvent) -> Result<(), SError> {
        match catch_unwind(move || (self.callback)(event)) {
            Ok(..) => Ok(()),
            Err(e) => Err(anyhow!(format!("Error while calling callback: {:?}", e)).into()),
        }
    }
}
