use super::{subscriber_name::SubscriberName, SubscriberEmail};

pub struct NewSubscriber {
    pub name: SubscriberName,
    pub email: SubscriberEmail,
}
