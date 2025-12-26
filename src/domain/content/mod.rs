use askama::filters::HtmlSafe;

use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone, Default, sqlx::Type)]
#[sqlx(transparent)]
pub struct MarkdownContent(String);

impl MarkdownContent {
    pub fn new(content: String) -> Self {
        Self(content)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for MarkdownContent {
    fn from(content: String) -> Self {
        Self(content)
    }
}

impl std::fmt::Display for MarkdownContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl HtmlSafe for MarkdownContent {}
