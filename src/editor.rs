use axum::{
    async_trait, 
    Json,
    http::StatusCode
};
use serde::{Serialize, Deserialize};
use tracing::Subscriber;

use tracing_serde::AsSerde;

#[derive(Deserialize)]
pub struct WriteBlog {
    pub author: String,
    pub title: String,
    pub content: String,
    pub author_id: u8,
}



#[derive(Serialize, Debug)]
pub struct ReadBlog {
    pub author: String,
    pub title: String,
    pub content: String,
}




#[async_trait]
pub trait AsyncBlog {
    async fn post_foo(&self, Json(payload) : Json<WriteBlog>) -> (StatusCode, Json<ReadBlog>);

}

#[async_trait]
impl AsyncBlog for WriteBlog {
    async fn post_foo(&self, Json(payload) : Json<WriteBlog>) -> (StatusCode, Json<ReadBlog>) {
        let poster = ReadBlog {
            author: payload.author,
            title: payload.title,
            content: payload.content,
        };

        (StatusCode::CREATED, Json(poster))
    }
}
