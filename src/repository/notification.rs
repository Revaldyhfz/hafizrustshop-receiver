use std::sync::RwLock;

use lazy_static::lazy_static;

use crate::model::notification::{self, Notification};

lazy_static! {
    pub static ref NOTIFICATION: RwLock<Vec<Notification>> = RwLock::new(Vec::new());
}

pub struct NotificationRepository;

impl NotificationRepository {
    pub fn add(notification: Notification) -> Notification{
        NOTIFICATION.write().unwrap().push(notification.clone());
        return notification;
    }

}