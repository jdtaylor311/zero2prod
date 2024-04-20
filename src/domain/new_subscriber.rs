//! src/domain/new_subscriber.rs
use super::SubscriberName;
use super::SubscriberEmail;

pub struct NewSubscriber {
    pub email: SubscriberEmail,
    pub name: SubscriberName
}
