use leptos::*;

#[server(Register, "/api")]
pub async fn register(context: Scope, username: String, password: String) -> Result<(), ServerFnError> {
  use crate::server::clients::postgre_sql_client::use_postgre_sql_client;
  use argon2::{ Argon2, PasswordHasher };
  use argon2::password_hash::SaltString;
  use argon2::password_hash::rand_core::OsRng;
  let client = use_postgre_sql_client(context)?;
  let salt = SaltString::generate(&mut OsRng);
  let password = Argon2::default().hash_password(password.as_bytes(), &salt).unwrap().to_string();
  client.user().create(
    username,
    password,
    vec![]
)
.exec()
.await?;
  Ok(())
}












































