use leptos::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct AccessToken {
    username: String,
    exp: usize,
}

#[server(Login, "/api")]
pub async fn login(username: String, password: String) -> Result<(), ServerFnError> {
  
  use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
  Ok(())
}














