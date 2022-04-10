use core::panic::{RefUnwindSafe, UnwindSafe};

pub trait SubscriberCallback<Event>: Fn(Event) + Send + UnwindSafe + RefUnwindSafe
where
    Event: crate::SubscriberEvent,
{
}
