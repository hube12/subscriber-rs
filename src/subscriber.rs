pub trait Subscriber<Event, Error: std::error::Error> {
    fn new<T: 'static + Fn(Event) + std::panic::UnwindSafe + std::panic::RefUnwindSafe>(
        callback: T,
    ) -> Self;
    fn notify(&self, event: Event) -> Result<(), Error>;
}
