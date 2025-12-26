use crate::prelude::*;

pub struct BlogService {
    db: DbPool,
}

impl BlogService {
    pub fn new(db: DbPool) -> Self {
        Self { db }
    }

    pub async fn find_by_id(&self, id: i64) -> Result<BlogPost> {
        let blog_post = sqlx::query_as!(BlogPost, "SELECT * FROM blog_posts WHERE id = ?", id)
            .fetch_one(&*self.db)
            .await?;
        Ok(blog_post)
    }

    pub async fn find_by_slug(&self, slug: &str) -> Result<BlogPost> {
        let blog_post = sqlx::query_as!(BlogPost, "SELECT * FROM blog_posts WHERE slug = ?", slug)
            .fetch_one(&*self.db)
            .await?;
        Ok(blog_post)
    }

    pub async fn find_all(&self) -> Result<Vec<BlogPost>> {
        let blog_posts = sqlx::query_as!(
            BlogPost,
            "SELECT * FROM blog_posts ORDER BY created_at DESC"
        )
        .fetch_all(&*self.db)
        .await?;
        Ok(blog_posts)
    }

    pub async fn create(&self, blog_post: &NewBlogPost) -> Result<BlogPost> {
        let id = sqlx::query_scalar!(
            "INSERT INTO blogs (author_id, title, content, image_url) VALUES (?, ?, ?, ?) RETURNING id",
            blog_post.author_id,
            blog_post.title,
            blog_post.content,
            blog_post.image_url,
        )
        .fetch_one(self.db.as_ref())
        .await?;

        self.find_by_id(id).await
    }

    pub async fn update(&self, blog_post: &EditBlogPost) -> Result<BlogPost> {
        let id = sqlx::query_scalar!(
            "UPDATE blogs SET title = ?, content = ?, image_url = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ? RETURNING id",
            blog_post.title,
            blog_post.content,
            blog_post.image_url,
            blog_post.id,
        )
        .fetch_one(self.db.as_ref())
        .await?;

        self.find_by_id(id).await
    }

    pub async fn delete(&self, id: i64) -> Result<()> {
        sqlx::query!("DELETE FROM blogs WHERE id = ?", id)
            .execute(self.db.as_ref())
            .await?;
        Ok(())
    }
}
