use cfg_if::cfg_if;

cfg_if! { if #[cfg(feature = "ssr")] {
    use leptos::{use_context, ServerFnError, Scope};
    use std::sync::Arc;
    use prisma_cli::prisma::PrismaClient;

    pub type ArcPrisma = Arc<PrismaClient>;

    pub async fn init_postgre_sql_client() -> ArcPrisma {
        let client = PrismaClient::_builder().build().await;
        let client = Arc::new(client.unwrap());
        client
    }

    pub fn use_postgre_sql_client(context: Scope) -> Result<ArcPrisma, ServerFnError> {
        use_context::<ArcPrisma>(context)
            .ok_or("postgre_sql_client is missing")
            .map_err(|error| ServerFnError::ServerError(error.to_string()))
    }
}}

