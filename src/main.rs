use axum::{routing::{get, post}, Router};

#[tokio::main]
async fn main(){

    let app = Router::new()
    .route("/", get(root))
    .route("/notes", post(create_note));
 
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();

}

async fn root() -> &'static str{
    "Hello, World!"
}

async fn create_note() -> &'static str{
    "Note created!"
}


