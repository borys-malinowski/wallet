use cfg_if::cfg_if;

cfg_if! { if #[cfg(feature = "ssr")] {
  use axum::extract::FromRef;
  use leptos::LeptosOptions;
  use crate::server::clients::postgre_sql_client::ArcPrisma;

  #[derive(FromRef, Debug, Clone)]
  pub struct AppState {
      pub leptos_options: LeptosOptions,
      pub postgre_sql_client: ArcPrisma,
  }
}}

