use axum::async_trait;
use serde_derive::Deserialize;


#[derive(Deserialize)]
pub struct Blog {
    pub author: &'static str,
    pub title: &'static str,
    pub author_id: u8,
}


#[async_trait]
pub trait AsyncBlog {
    async fn get_foo() -> Blog;
}

#[async_trait]
impl AsyncBlog for Blog {
    async fn get_foo() -> Blog {
        Blog {
            author: "Jung",
            title: "이 것이 시작이다.",
            author_id: 1,
        }
    }
}