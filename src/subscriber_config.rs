pub trait SubscriberConfig: Default {
    fn subscriber_count(&self) -> usize;
    fn channel_size(&self) -> usize;
}
