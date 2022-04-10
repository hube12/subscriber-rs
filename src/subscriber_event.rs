pub trait SubscriberEvent: Clone + Send + Sync {
    type Type: crate::SubscriberEventType;

    fn should_kill(&self) -> bool;
    fn get_type(&self) -> Self::Type;
}
