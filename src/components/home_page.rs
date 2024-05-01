use crate::server::api::rates::rates;
use leptos::*;

#[component]
pub fn HomePage(context: Scope) -> impl IntoView {
    let (value, set_value) = create_signal(context, 0 as f32);
    let (quantity, set_quantity) = create_signal(context, 0 as f32);
    let (price, set_price) = create_signal(context, 0 as f64);
    let (transaction_value, set_transaction_value) = create_signal(context, 0 as f64);
    let (isin, set_isin) = create_signal(context, String::new());
    let (multiply_score, set_multiply_score) = create_signal(context, 0 as f32);
    create_effect(context, move |_| {
        set_multiply_score(quantity.get() * value.get());
    });
    view! { context,
      <div style="padding: 10px">
        <span>You are logged as User:</span>
      </div>
      <div style="padding: 10px">
        <span>Lets add shares to portfolio</span>
      </div>
      <div style="display: flex;
      justify-content: center;
      align-items: center; padding: 5px">
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

        <input
          class="py-3 px-5 block w-full border-gray-200 rounded-full text-sm focus:border-blue-500 focus:ring-blue-500 dark:bg-slate-900 dark:border-gray-700 dark:text-gray-400"
          id="quantity"
          min="0"
          type="number"
          placeholder="Paste here your quantity of shares"
          on:input=move |event| {
              let value = event_target_value(&event);
              let value = value.parse::<f64>();
              match value {
                  Ok(value) => set_price(value),
                  Err(_) => {}
              };
          }
        />

        <input
          class="py-3 px-5 block w-full border-gray-200 rounded-full text-sm focus:border-blue-500 focus:ring-blue-500 dark:bg-slate-900 dark:border-gray-700 dark:text-gray-400"
          id="value"
          type="number"
          minlength="12"
          maxlength="12"
          placeholder="Paste here share value"
          on:input=move |event| {
              let value = event_target_value(&event);
              let value = value.parse::<f64>();
              match value {
                  Ok(value) => set_transaction_value(value),
                  Err(_) => {}
              };
          }
        />

        <input
          class="py-3 px-5 block w-full border-gray-200 rounded-full text-sm focus:border-blue-500 focus:ring-blue-500 dark:bg-slate-900 dark:border-gray-700 dark:text-gray-400"
          id="date"
          type="date"
          minlength="12"
          maxlength="12"
          placeholder="Paste here buy date"
        />

        <input
          class="py-3 px-5 block w-full border-gray-200 rounded-full text-sm focus:border-blue-500 focus:ring-blue-500 dark:bg-slate-900 dark:border-gray-700 dark:text-gray-400"
          id="price"
          minlength="12"
          maxlength="12"
          placeholder="Paste here buy price"
          on:input=move |event| {
              let value = event_target_value(&event);
              let value = value.parse::<f32>();
              match value {
                  Ok(value) => set_quantity(value),
                  Err(_) => {}
              };
          }
        />

      </div>
      <button
        class="py-3 px-4 inline-flex justify-center items-center gap-2 rounded-md bg-blue-100 border border-transparent font-semibold text-blue-500 hover:text-white hover:bg-blue-500 focus:outline-none focus:ring-2 ring-offset-white focus:ring-blue-500 focus:ring-offset-2 transition-all text-sm"
        on:click=move |_| {
            spawn_local(async move {
                let text = rates(
                        context,
                        isin.get(),
                        price.get(),
                        quantity.get(),
                        transaction_value.get(),
                    )
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

        "Add"
      </button>
      <div>{multiply_score}</div>
    }
}

