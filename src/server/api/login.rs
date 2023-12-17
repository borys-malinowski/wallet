use leptos::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct AccessToken {
    username: String,
    exp: usize,
}

#[server(Login, "/api")]
pub async fn login(context: Scope, form_username: String, password: String) -> Result< (), ServerFnError> {
  use crate::server::clients::postgre_sql_client::use_postgre_sql_client;
  use prisma_cli::prisma::user::username;
  use http::StatusCode;
  let client = use_postgre_sql_client(context).unwrap();
  let database_user = client.user().find_unique(username::equals(form_username.to_string())).exec().await.unwrap();
  use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
  Ok(())
}

