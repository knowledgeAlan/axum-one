use axum::body::Body;
use axum::{extract::Request, middleware::Next, response::Response};
use lazy_regex::regex_captures;
use tower_cookies::Cookies;

use crate::web::AUTH_TOKEN;
use crate::{Error,Result};
 pub async fn mw_require_auth(
    cookies:Cookies,
    req:Request<Body>,
    next:Next,
 ) -> Result<Response>{

    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    auth_token.ok_or(Error::AuthFailNoAuthTokenCookie)?;
    Ok(next.run(req).await)
 }


 fn parse_token(token:String) -> Result<(u64,String,String)> {

    let (_whole,user_id,exp,sign) = regex_captures!(
        r#"^user-(\d+)\.(.+)\.(.+)"#,&token
    ).ok_or(Error::AuthFailTokenWrongFormat)?;
    todo!()
 }