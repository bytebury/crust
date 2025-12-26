use crate::{AppInfo, prelude::*};

pub mod admin;
pub mod auth;
pub mod blog;
pub mod homepage;
pub mod payments;
pub mod webhooks;

pub struct SharedContext {
    pub app_info: AppInfo,
    pub current_user: Option<User>,
    pub announcement: Option<Announcement>,
}
impl SharedContext {
    pub fn new(app_info: &AppInfo, user: Option<User>) -> Self {
        Self {
            app_info: app_info.clone(),
            current_user: user,
            announcement: None,
        }
    }

    pub fn new_with_announcement(
        app_info: &AppInfo,
        user: Option<User>,
        announcement: &Option<Announcement>,
    ) -> Self {
        Self {
            app_info: app_info.clone(),
            current_user: user,
            announcement: announcement.clone(),
        }
    }
}
