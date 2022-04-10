#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]

#[cfg(all(not(feature = "std"), not(test)))]
extern crate core as std;

#[cfg(feature = "alloc")]
extern crate alloc;

pub use subscriber::Subscriber;
pub use subscriber_callback::SubscriberCallback;
pub use subscriber_config::SubscriberConfig;
pub use subscriber_error::SubscriberError;
pub use subscriber_event::SubscriberEvent;
pub use subscriber_event_type::SubscriberEventType;
pub use subscriber_server::SubscriberServer;
pub use subscriber_server_handle::{EventHandle, SubscribeHandle, SubscriberServerHandle};
pub use subscribers::Subscribers;

mod subscriber;
mod subscriber_callback;
mod subscriber_config;
mod subscriber_error;
mod subscriber_event;
mod subscriber_event_type;
mod subscriber_server;
mod subscriber_server_handle;
mod subscribers;
