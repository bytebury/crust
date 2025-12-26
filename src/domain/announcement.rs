use crate::prelude::*;

#[derive(Deserialize, Serialize, Clone, FromRow)]
pub struct Announcement {
    pub id: i64,
    pub content: MarkdownContent,
    pub active: Checkbox,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Default for Announcement {
    fn default() -> Self {
        Self {
            id: i64::default(),
            content: MarkdownContent::default(),
            active: Checkbox::checked(),
            created_at: NaiveDateTime::default(),
            updated_at: NaiveDateTime::default(),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct NewAnnouncement {
    pub content: MarkdownContent,
    pub active: Option<Checkbox>,
}

impl Default for NewAnnouncement {
    fn default() -> Self {
        Self {
            content: MarkdownContent::default(),
            active: Some(Checkbox::checked()),
        }
    }
}

#[derive(Deserialize, Serialize, Default)]
pub struct EditAnnouncement {
    pub id: i64,
    pub content: MarkdownContent,
    pub active: Option<Checkbox>,
}

impl From<Announcement> for EditAnnouncement {
    fn from(value: Announcement) -> Self {
        Self {
            id: value.id,
            content: value.content,
            active: Some(value.active),
        }
    }
}
