
use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::{web,Error,Result};

pub fn routes() -> Router {
    Router::new().route("/api/login",post(api_login))
}


async fn api_login(playload:Json<LoginPayLoad>) -> Result<Json<Value>>{
    println!("->> {:<12} -api_login","handler");

    if(playload.username !="demo1" || playload.pwd != "welcome"){
        return  Err(Error::LoginFail);
    }


    let body = Json(json!({
        "result":{
            "success":true,
        }
    }));

    Ok(body)
}

#[derive(Debug,Deserialize)]
struct LoginPayLoad {

    username: String,
    pwd: String,
}
