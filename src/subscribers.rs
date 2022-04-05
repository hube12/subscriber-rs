use crate::Subscriber;

#[derive(Debug)]
pub struct Subscribers<Sub> {
    subscribers: Vec<Sub>,
}

// pub trait CallBack<Event>:
//     Fn(Event) + std::panic::UnwindSafe + std::panic::RefUnwindSafe + std::fmt::Debug
// {
// }

impl<Sub> Subscribers<Sub> {
    pub fn new() -> Self {
        Self {
            subscribers: Vec::with_capacity(3),
        }
    }

    pub fn push<Event: Clone, Error: std::error::Error>(&mut self, subscriber: Sub)
    where
        Sub: Subscriber<Event, Error>,
    {
        self.subscribers.push(subscriber);
    }

    pub fn notify<Event: Clone, Error: std::error::Error>(&self, event: Event) -> Option<Vec<Error>>
    where
        Sub: Subscriber<Event, Error>,
    {
        if self.subscribers.len() == 1 {
            match self.subscribers.first().unwrap().notify(event) {
                Ok(..) => None,
                Err(e) => Some(vec![e]),
            }
        } else {
            let mut errors = Vec::with_capacity(self.subscribers.len());
            for sub in &self.subscribers {
                if let Err(e) = sub.notify(event.clone()) {
                    errors.push(e)
                }
            }
            if errors.len() > 0 {
                Some(errors)
            } else {
                None
            }
        }
    }
}
