pub trait SubscriberConfig: Default + Send {
    fn subscriber_count(&self) -> usize;
    fn channel_size(&self) -> usize;
}
