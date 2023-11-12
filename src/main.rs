use cfg_if::cfg_if;

cfg_if! { if #[cfg(feature = "ssr")] {
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes, handle_server_fns_with_context};
    use wallet::{
        app::*, 
        server::{
            types::app_state::AppState,
            clients::postgre_sql_client::{init_prisma_client},
        },
        fileserv::file_and_error_handler,
    };
    use axum::{
        Router, 
        response::{Response, IntoResponse},
        routing::get,
        extract::{State, Path, RawQuery},
        http::{Request, header::HeaderMap},
        body::Body as AxumBody,
    };
    
    async fn server_fn_handler(State(app_state): State<AppState>, path: Path<String>, headers: HeaderMap, raw_query: RawQuery,
    request: Request<AxumBody>) -> impl IntoResponse {
        handle_server_fns_with_context(path, headers, raw_query, move |context| {
            provide_context(context, app_state.postgre_sql_client.clone());
        }, request).await
    }

    async fn leptos_routes_handler(State(app_state): State<AppState>, req: Request<AxumBody>) -> Response {
            let handler = leptos_axum::render_app_to_stream_with_context(app_state.leptos_options.clone(),
            move |cx| {
                provide_context(cx, app_state.postgre_sql_client.clone());
            },
            |cx| view! { cx, <App/> }
        );
        handler(req).await.into_response()
    }
}}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");
    let configuration = get_configuration(None).await.unwrap();
    let leptos_options = configuration.leptos_options;
    let site_addr = leptos_options.site_addr;
    let routes = generate_route_list(|cx| view! { cx, <App/> }).await;
    let postgre_sql_client = init_prisma_client().await;
    let app_state = AppState {
        leptos_options,
        postgre_sql_client: postgre_sql_client.clone()
    };
    let app = Router::new()
        .route("/api/*fn_name", get(server_fn_handler).post(server_fn_handler)) // zobaczyć czy można bez get
        .leptos_routes_with_handler(routes, get(leptos_routes_handler) )
        .fallback(file_and_error_handler)
        .with_state(app_state);
    log!("listening on http://{}", &site_addr);
    axum::Server::bind(&site_addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[cfg(feature = "ssr")]
#[cfg(not(feature = "ssr"))]
pub fn main() {}

