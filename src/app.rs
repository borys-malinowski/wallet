use crate::{
    error_template::{AppError, ErrorTemplate},
    server::{api::login::login, api::rates::rates},
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(context: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(context);

    view! { context,
      // injects a stylesheet into the document <head>
      // id=leptos means cargo-leptos will hot-reload this stylesheet
      <Stylesheet id="leptos" href="/pkg/wallet.css"/>
      <Script src="./preline/preline.js"/>

      // sets the document title
      <Title text="Welcome to Leptos"/>

      // content for this welcome page
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
          </Routes>
        </main>
      </Router>
    }
}

#[component]
fn HomePage(context: Scope) -> impl IntoView {
    let (value, set_value) = create_signal(context, 0 as f32);
    let (quantity, set_quantity) = create_signal(context, 0 as f32);
    let (isin, set_isin) = create_signal(context, String::new());
    let (multiply_score, set_multiply_score) = create_signal(context, 0 as f32);
    create_effect(context, move |_| {
        set_multiply_score(quantity.get() * value.get());
    });
    view! { context,
      <button
        class="py-3 px-4 inline-flex justify-center items-center gap-2 rounded-md bg-blue-100 border border-transparent font-semibold text-blue-500 hover:text-white hover:bg-blue-500 focus:outline-none focus:ring-2 ring-offset-white focus:ring-blue-500 focus:ring-offset-2 transition-all text-sm"
        on:click=move |_| {
            spawn_local(async move {
                let text = rates(isin.get(), quantity.get()).await.unwrap();
                let text = text.replace(',', ".");
                let value = text.parse::<f32>();
                match value {
                    Ok(value) => set_value(value),
                    Err(_) => {}
                };
            });
        }
      >

        "Check rate"
        {value}
      </button>
      <input
        class="py-3 px-5 block w-full border-gray-200 rounded-full text-sm focus:border-blue-500 focus:ring-blue-500 dark:bg-slate-900 dark:border-gray-700 dark:text-gray-400"
        id="quantity"
        min="0"
        type="number"
        placeholder="Paste here your quantity of shares"
        on:input=move |event| {
            let value = event_target_value(&event);
            let value = value.parse::<f32>();
            match value {
                Ok(value) => set_quantity(value),
                Err(_) => {}
            };
        }
      />

      <input
        class="py-3 px-5 block w-full border-gray-200 rounded-full text-sm focus:border-blue-500 focus:ring-blue-500 dark:bg-slate-900 dark:border-gray-700 dark:text-gray-400"
        id="isin"
        minlength="12"
        maxlength="12"
        placeholder="Paste here share isin"
        on:input=move |event| {
            let value = event_target_value(&event);
            set_isin(value);
        }
      />

      <div>{multiply_score}</div>
    }
}

#[component]
fn LoginPage(context: Scope) -> impl IntoView {
    let (username, set_username) = create_signal(context, String::new());
    let (password, set_password) = create_signal(context, String::new());
    view! { context,
      <form>
        <input
          class="py-3 px-5 block w-full border-gray-200 rounded-full text-sm focus:border-blue-500 focus:ring-blue-500 dark:bg-slate-900 dark:border-gray-700 dark:text-gray-400"
          id="username"
          placeholder="Paste your username here"
          on:input=move |event| {
              let value = event_target_value(&event);
              set_username(value);
          }
        />

        <input
          type="password"
          class="py-3 px-5 block w-full border-gray-200 rounded-full text-sm focus:border-blue-500 focus:ring-blue-500 dark:bg-slate-900 dark:border-gray-700 dark:text-gray-400"
          id="password"
          placeholder="Paste your password here"
          on:input=move |event| {
              let value = event_target_value(&event);
              set_password(value);
          }
        />

        <button
          type="submit"
          class="py-3 px-4 inline-flex justify-center items-center gap-2 rounded-md bg-blue-100 border border-transparent font-semibold text-blue-500 hover:text-white hover:bg-blue-500 focus:outline-none focus:ring-2 ring-offset-white focus:ring-blue-500 focus:ring-offset-2 transition-all text-sm"
          on:submit=move |_| {
              spawn_local(async move {
                  let username = username.get();
                  let password = password.get();
                  login(username, password).await.unwrap();
              });
          }
        >

          Login
        </button>
      </form>
    }
}

