use core::hash::Hash;

pub trait SubscriberEventType: Hash + Eq + Send + Sync + Ord {}
