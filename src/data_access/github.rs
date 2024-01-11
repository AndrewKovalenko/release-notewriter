use crate::application::dtos::release::Release;
use chrono;

pub struct Repository<'a> {
    url: &'a str,
}

impl<'a> Repository<'a> {
    pub fn get_latest_release(&self) -> Release {
        return Release {
            timestamp: chrono::offset::Utc::now(),
        };
    }
}
