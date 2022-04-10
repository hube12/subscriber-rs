pub trait Subscriber<Event, Error>: Send
where
    Error: crate::SubscriberError,
    Event: crate::SubscriberEvent,
{
    fn new<T: crate::SubscriberCallback<Event> + 'static>(callback: T) -> Self;
    fn notify(&self, event: Event) -> Result<(), Error>;
}
