use crate::server::api::login::login;
use leptos::*;

#[component]
pub fn LoginPage(context: Scope) -> impl IntoView {
    let (username, set_username) = create_signal(context, String::new());
    let (password, set_password) = create_signal(context, String::new());
    let (user, set_user) = create_signal(context, String::new());
    provide_context(context, user);
    let (authorization, set_authorization) = create_signal(context, false);
    create_effect(context, move |_| {
        if authorization.get() {
            let navigate = leptos_router::use_navigate(context);
            request_animation_frame(move || {
                _ = navigate("/shares", Default::default());
            });
        }
    });
    view! { context,
      <div>
        <span>LOGIN PAGE</span>
      </div>
      <div>
        <span>LOGGED as {user}</span>
      </div>
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

        <div
          class="py-3 px-4 inline-flex justify-center items-center gap-2 rounded-md bg-blue-100 border border-transparent font-semibold text-blue-500 hover:text-white hover:bg-blue-500 focus:outline-none focus:ring-2 ring-offset-white focus:ring-blue-500 focus:ring-offset-2 transition-all text-sm"
          on:click=move |_| {
              spawn_local(async move {
                  let username = username.get();
                  let password = password.get();
                  let result = login(context, username, password).await;
                  if result.is_ok() {
                      set_user(result.clone().unwrap());
                      set_authorization.set(true);
                  }
              });
          }
        >

          Login
        </div>
        <div
          class="py-3 px-4 inline-flex justify-center items-center gap-2 rounded-md bg-blue-100 border border-transparent font-semibold text-blue-500 hover:text-white hover:bg-blue-500 focus:outline-none focus:ring-2 ring-offset-white focus:ring-blue-500 focus:ring-offset-2 transition-all text-sm"
          on:click=move |_| {
              spawn_local(async move {
                  let navigate = leptos_router::use_navigate(context);
                  request_animation_frame(move || {
                      _ = navigate("/register", Default::default());
                  });
              });
          }
        >

          Register
        </div>
      </form>
    }
}

