use anyhow::anyhow;
use std::fmt::{Debug, Display, Formatter};
use std::panic::{catch_unwind, UnwindSafe};
use subscriber_rs::{SubscriberServer};

fn main() {
    let mut server: SubscriberServer<
        EventType,
        SubscriberEvent,
        SubscriberConfig,
        Subscriber<SubscriberEvent>,
    > = SubscriberServer::new(SubscriberConfig::default());
    // server.run()
    server.subscribe(EventType::ErrorProcesses, |e| println!("{:?}", e));
    server.send(
        EventType::ErrorProcesses,
        SubscriberEvent::ErrorProcesses(String::from("test")),
    );
}

#[derive(Clone, Debug)]
pub enum SubscriberEvent {
    ErrorChannel(String),
    ErrorProcesses(String),
    Event(String),
    Result(String),
}

impl UnwindSafe for SubscriberEvent {}

#[derive(Hash, PartialOrd, PartialEq, Eq, Debug)]
pub enum EventType {
    /// Errors in channels map to (0,0xf(
    ErrorChannels(u8),
    /// Error in processes map to 0xf
    ErrorProcesses,
    /// ResultMessages map to 0x10
    ResultMessages,
    /// Events map to (0x7fff,0xffff)
    Event(u16),
}

impl EventType {
    const EVENT_MASK: u16 = 0x7FFF;
    const ERROR_CHANNELS_MASK: u16 = 0xF;
    const ERROR_PROCESSES: u16 = 0x10;
    const RESULT_MESSAGES: u16 = 0x11;
}

#[derive(Debug)]
pub struct SubscriberConfig {
    channel_size: u32,
}

impl subscriber_rs::SubscriberConfig for SubscriberConfig {
    fn subscriber_count(&self) -> usize {
        todo!()
    }

    fn channel_size(&self) -> usize {
        todo!()
    }
}

impl Default for SubscriberConfig {
    fn default() -> Self {
        Self { channel_size: 1024 }
    }
}

impl TryFrom<u16> for EventType {
    type Error = anyhow::Error;

    fn try_from(n: u16) -> Result<Self, Self::Error> {
        Ok(match n {
            0..=Self::ERROR_CHANNELS_MASK => {
                Self::ErrorChannels((n & Self::ERROR_CHANNELS_MASK) as u8)
            }
            Self::ERROR_PROCESSES => Self::ErrorProcesses,
            Self::RESULT_MESSAGES => Self::ResultMessages,
            // Not used range
            Self::EVENT_MASK..=u16::MAX => Self::Event(n & Self::EVENT_MASK),
            _ => {
                return Err(anyhow!("Not a valid Event type"));
            }
        })
    }
}

impl Into<u16> for EventType {
    fn into(self) -> u16 {
        match self {
            EventType::ErrorChannels(side) => side.into(),
            EventType::ErrorProcesses => Self::ERROR_PROCESSES,
            EventType::ResultMessages => Self::RESULT_MESSAGES,
            EventType::Event(n) => n | Self::EVENT_MASK,
        }
    }
}

pub struct Subscriber<Event> {
    callback: Box<dyn Fn(Event) + std::panic::UnwindSafe + std::panic::RefUnwindSafe>,
}

#[derive(Debug)]
pub struct MyError {
    msg: String,
}

impl From<String> for MyError {
    fn from(msg: String) -> Self {
        Self { msg }
    }
}

impl Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for MyError {}

impl subscriber_rs::Subscriber<SubscriberEvent, MyError> for Subscriber<SubscriberEvent> {
    fn new<
        T: 'static + Fn(SubscriberEvent) + std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    >(
        callback: T,
    ) -> Self {
        Self {
            callback: Box::new(callback),
        }
    }

    fn notify(&self, event: SubscriberEvent) -> Result<(), MyError> {
        match catch_unwind(move || (self.callback)(event)) {
            Ok(..) => Ok(()),
            Err(e) => Err(MyError::from(format!(
                "Error while calling callback: {:?}",
                e
            ))),
        }
    }
}
