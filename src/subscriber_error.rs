#[cfg(feature = "std")]
pub trait SubscriberError: std::error::Error + Send + std::fmt::Debug {}
#[cfg(not(feature = "std"))]
pub trait SubscriberError: Send {}
