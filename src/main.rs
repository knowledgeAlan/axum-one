 

use axum::{extract::{Path, Query}, response::{Html, IntoResponse}, routing::{get, get_service, Router}};
use serde::Deserialize;
use tower_http::services::ServeDir;
use std::net::SocketAddr;
pub use self::error::{Error, Result};
mod error;
mod web;
 
#[derive(Debug,Deserialize)]
struct HelloParams {
    name: Option<String>,
}

#[tokio::main]
async fn main() {
    let routes_hello = Router::new()
    .merge(routes_hello())
    .merge(web::routes_login::routes())
    .fallback_service(routes_static())
    ;


    let addr = SocketAddr::from(([127, 0, 0, 1],8080));

   

    axum_server::bind(addr)
                .serve(routes_hello.into_make_service())
                .await
                .unwrap();

}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

fn routes_hello() -> Router {
    Router::new()  
    .route("/hello",get(handler_hello))
    .route("/hello2/:name", get(handler_hello2))
}

async fn handler_hello(Query(params):Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}","handler");

    let name =  params.name.as_deref().unwrap_or("world");
    return Html(format!("Hello {name} world"));
}

async fn handler_hello2(Path(name) : Path<String>) -> impl IntoResponse{
    println!("->> {:12} -handler_hello2 - {name:?}","handler");

    Html(format!("hello2 {name}"))
}