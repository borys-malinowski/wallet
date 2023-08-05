use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/wallet.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|cx| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { cx,
                <ErrorTemplate outside_errors/>
            }
            .into_view(cx)
        }>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

#[server(Rates, "/api")]
pub async fn rates() -> Result<String, ServerFnError> {
    use fantoccini::{ClientBuilder, Locator};
    let connection = ClientBuilder::native()
        .connect("http://localhost:4444")
        .await
        .expect("failed to connect to WebDriver");
    connection
        .goto("https://www.gpw.pl/spolka?isin=PLDINPL00011")
        .await?;
    let text = connection
        .find(Locator::Css(".summary"))
        .await?
        .text()
        .await?;
    Ok(text)
}
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    let (value, set_value) = create_signal(cx, String::new());
    view! { cx,
        <button on:click=move |_| {
            spawn_local(async move {
                let text = rates().await.unwrap();
                set_value(text)
            });
        }>
            "Add Todo"
            {value}
        </button>
    }
}
