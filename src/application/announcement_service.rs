use crate::prelude::*;

pub struct AnnouncementService {
    db: DbPool,
}

impl AnnouncementService {
    pub fn new(db: DbPool) -> Self {
        Self { db }
    }

    pub async fn find_latest(&self) -> Option<Announcement> {
        sqlx::query_as!(
            Announcement,
            "SELECT * FROM announcements WHERE active = true ORDER BY created_at DESC LIMIT 1"
        )
        .fetch_optional(&*self.db)
        .await
        .unwrap_or(None)
    }

    pub async fn find_by_id(&self, id: i64) -> Result<Announcement> {
        let announcement =
            sqlx::query_as!(Announcement, "SELECT * FROM announcements WHERE id = ?", id)
                .fetch_one(&*self.db)
                .await?;
        Ok(announcement)
    }

    pub async fn find_all(&self) -> Result<Vec<Announcement>> {
        let announcements = sqlx::query_as!(
            Announcement,
            "SELECT * FROM announcements ORDER BY created_at DESC"
        )
        .fetch_all(&*self.db)
        .await?;
        Ok(announcements)
    }

    pub async fn create(&self, announcement: &NewAnnouncement) -> Result<Announcement> {
        let active = announcement.active.clone().unwrap_or_default().is_checked();
        let id = sqlx::query_scalar!(
            "INSERT INTO announcements (content, active) VALUES (?, ?) RETURNING id",
            announcement.content,
            active,
        )
        .fetch_one(self.db.as_ref())
        .await?;

        self.find_by_id(id).await
    }

    pub async fn update(&self, announcement: &EditAnnouncement) -> Result<Announcement> {
        let active = announcement.active.clone().unwrap_or_default().is_checked();
        let id = sqlx::query_scalar!(
            "UPDATE announcements SET content = ?, active = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ? RETURNING id",
            announcement.content,
            active,
            announcement.id,
        )
        .fetch_one(self.db.as_ref())
        .await?;

        self.find_by_id(id).await
    }

    pub async fn delete(&self, id: i64) -> Result<()> {
        sqlx::query!("DELETE FROM announcements WHERE id = ?", id)
            .execute(self.db.as_ref())
            .await?;
        Ok(())
    }
}
