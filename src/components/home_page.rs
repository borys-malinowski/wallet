use crate::server::api::rates::rates;
use leptos::*;

#[component]
pub fn HomePage(context: Scope) -> impl IntoView {
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
                let text = rates(context, isin.get(), quantity.get())
                    .await
                    .unwrap();
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

