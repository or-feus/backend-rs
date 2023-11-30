mod editor;

use tracing::info;

use axum::{
    routing::{get, post},
    Router,
    Json,
    http::StatusCode
};

use editor::{WriteBlog, ReadBlog};
// use backend_rs::editor::{WriteBlog};


#[tokio::main]
async fn main() {


    // you must use "dotenv".

    tracing_subscriber::fmt::init();

    let test_log = 3;

    info!(test_log, "ok! ok!");



    // build our application with a single route
    let app = Router::new()
        .route("/", get(root))
        .route("/foo", get(get_foo).post(post_foo))
        .route("/foo/bar", get(foo_bar));

    // route_layer, with_state, layer, nest, nest_service in Router
    // what were these?



    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    info!("root");

    "root"

}


async fn post_foo(Json(payload): Json<WriteBlog>) -> (StatusCode, Json<ReadBlog>) {
    
    let poster = ReadBlog {
        author: payload.author,
        title: payload.title,
        content: payload.content,
    };
    info!("Logging JSON: {:#?}", poster);

    (StatusCode::CREATED, Json(poster))

}



async fn get_foo() -> (StatusCode, Json<ReadBlog>) {
    let poster = ReadBlog {
        author: String::from("정선교"),
        title: String::from("제목"),
        content: String::from("본문"),
    };

    info!("Logging JSON: {:#?}", poster);



    (StatusCode::CREATED, Json(poster))
}

async fn foo_bar() -> &'static str {
    info!("foo_bar");
    "foo_bar"
}