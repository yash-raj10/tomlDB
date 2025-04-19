use std::net::SocketAddr;
mod routes;
mod fns;
use crate::routes::routess;

#[tokio::main]
async fn main() {
   let app = routess();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
  
}

