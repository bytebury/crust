use crate::prelude::*;

#[derive(Deserialize, Serialize, Clone, FromRow)]
pub struct BlogPost {
    pub id: i64,
    pub author_id: i64,
    pub author_name: String,
    pub author_image_url: String,
    pub slug: String,
    pub title: String,
    pub content: MarkdownContent,
    pub image_url: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Serialize, Default)]
pub struct NewBlogPost {
    pub author_id: i64,
    pub title: String,
    pub content: MarkdownContent,
    pub image_url: String,
}

#[derive(Deserialize, Serialize, Default)]
pub struct EditBlogPost {
    pub id: i64,
    pub title: String,
    pub content: MarkdownContent,
    pub image_url: String,
}

impl From<BlogPost> for EditBlogPost {
    fn from(value: BlogPost) -> Self {
        Self {
            id: value.id,
            title: value.title,
            content: value.content,
            image_url: value.image_url,
        }
    }
}
