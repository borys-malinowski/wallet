use leptos::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct AccessToken {
    username: String,
    exp: usize,
}

#[server(Login, "/api")]
pub async fn login(context: Scope, form_username: String, form_password: String) -> Result< String, ServerFnError> {
  use crate::server::clients::postgre_sql_client::use_postgre_sql_client;
  use prisma_cli::prisma::user::username;
  use bcrypt::{verify};
  let client = use_postgre_sql_client(context)?;
  let username = form_username.to_string();
  let password = form_password.to_string();
  let db_user = client.user().find_unique(username::equals(username.clone())).exec().await?;
  match db_user {
    Some(db_user)=>{
      let xd = verify(password.clone(), &db_user.password);
      match xd {Ok(v)=>{println!("working with version: {v:?}")}
    Err(_) => todo!(), }
    }
    None => todo!(), 
  }
  use jsonwebtoken::{encode, Header, Algorithm, EncodingKey};
  use chrono::{Utc, Duration};
  let access_expiration = Utc::now()
    .checked_add_signed(Duration::minutes(10))
    .expect("valid timestamp")
    .timestamp();
  let refresh_expiration_duration = Duration::days(1);
  let refresh_expiration = Utc::now()
    .checked_add_signed(refresh_expiration_duration)
    .expect("valid timestamp")
    .timestamp();
  let access_body = AccessToken {
    username: username.clone(),
    exp: access_expiration as usize,
  };
  let refresh_body = AccessToken {
    username,
    exp: refresh_expiration as usize,
  };
  let header = Header::new(Algorithm::HS512);
  let access_token = encode(&header, &access_body, &EncodingKey::from_secret(dotenv!("ACCESS_TOKEN_SECRET").as_bytes()))?;
  let refresh_token = encode(&header, &refresh_body, &EncodingKey::from_secret(dotenv!("REFRESH_TOKEN_SECRET").as_bytes()))?;
  use cookie::{Cookie, time::Duration as CookieDuration, SameSite};
  let cookie = Cookie::build(("refresh-token", refresh_token))
    .secure(true)
    .path("/")
    .same_site(SameSite::None)
    .max_age(CookieDuration::days(1))
    .http_only(true);
  use leptos_axum::ResponseOptions;
  use axum::http::header::{SET_COOKIE,HeaderValue};
  let response = expect_context::<ResponseOptions>(context);
  response.insert_header(SET_COOKIE, HeaderValue::from_str(&cookie.to_string())?);
  Ok(access_token)
}























