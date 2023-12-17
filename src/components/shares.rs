use crate::server::api::get_rates::get_rates;
use leptos::*;

#[component]
pub fn Shares(context: Scope) -> impl IntoView {
    let (value, set_value) = create_signal(context, vec![]);
    // create_effect(context, move |_| {
    //     set_multiply_score(quantity.get() * value.get());
    // });
    view! { context,
      <button
        type="submit"
        class="py-3 px-4 inline-flex justify-center items-center gap-2 rounded-md bg-blue-100 border border-transparent font-semibold text-blue-500 hover:text-white hover:bg-blue-500 focus:outline-none focus:ring-2 ring-offset-white focus:ring-blue-500 focus:ring-offset-2 transition-all text-sm"
        on:click=move |_| {
            spawn_local(async move {
                let data = get_rates(context).await.unwrap();
                set_value(data);
            });
        }
      >

        Get Shares
      </button>
      <div>{format!("{:#?}", value.get())}</div>
    }
}

