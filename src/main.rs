 

use axum::{extract::Query, response::{Html, IntoResponse}, routing::{get, Router}};
use serde::Deserialize;
use std::net::SocketAddr;
 

#[derive(Debug,Deserialize)]
struct HelloParams {
    name: Option<String>,
}

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route("/hello",get(handler_hello));


    let addr = SocketAddr::from(([127, 0, 0, 1],8080));

   

    axum_server::bind(addr)
                .serve(routes_hello.into_make_service())
                .await
                .unwrap();

}

async fn handler_hello(Query(params):Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}","handler");

    let name =  params.name.as_deref().unwrap_or("world");
    return Html(format!("Hello {name} world"));
}