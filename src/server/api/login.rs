use leptos::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct AccessToken {
    username: String,
    exp: usize,
}

#[server(Login, "/api")]
pub async fn login(context: Scope, form_username: String, form_password: String) -> Result<String, ServerFnError> {
  use crate::server::clients::postgre_sql_client::use_postgre_sql_client;
  use prisma_cli::prisma::user::username;
  use argon2::{ Argon2, PasswordHash, PasswordVerifier};
  let client = use_postgre_sql_client(context)?;
  let username = form_username.to_string();
  let cloned_username = username.clone();
  let db_user = client.user().find_unique(username::equals(username.clone())).exec().await?;
  if db_user.is_none() {
    return Err(ServerFnError::ServerError(
      String::from("bad request")
    ));
  } 
  let db_user = db_user.unwrap();
  let parsed_hash = PasswordHash::new(&db_user.password).unwrap();
  let is_valid = Argon2::default()
        .verify_password(form_password.as_bytes(), &parsed_hash)
        .map_or(false, |_| true);
  if !is_valid {
    return Err(ServerFnError::ServerError(
      String::from("forbidden password")
    ));
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
  encode(&header, &access_body, &EncodingKey::from_secret(dotenv!("ACCESS_TOKEN_SECRET").as_bytes()))?;
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
  Ok(cloned_username)
}

