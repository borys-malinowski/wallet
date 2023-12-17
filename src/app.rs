use crate::{
    components::{home_page::HomePage, login_page::LoginPage, shares::Shares},
    error_template::{AppError, ErrorTemplate},
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[component]
pub fn App(context: Scope) -> impl IntoView {
    provide_meta_context(context);
    view! { context,
      <Stylesheet id="leptos" href="/pkg/wallet.css"/>
      <Script src="./preline/preline.js"/>
      <Title text="Welcome to Leptos"/>
      <Router fallback=|context| {
          let mut outside_errors = Errors::default();
          outside_errors.insert_with_default_key(AppError::NotFound);
          view! { context, <ErrorTemplate outside_errors/> }.into_view(context)
      }>
        <main>
          <Routes>
            <Route path="" view=|context| view! { context, <HomePage/> }/>
            <Route
              path="/login"
              view=|context| view! { context, <LoginPage/> }
            />
            <Route
              ssr=SsrMode::Async
              path="/shares"
              view=|context| {
                  view! { context, <Shares/> }
              }
            />

          </Routes>
        </main>
      </Router>
    }
}

