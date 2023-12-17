use crate::server::api::get_rates::{get_rates, ShareGroup};
use leptos::{html::Div, *};

#[component]
pub fn Shares(context: Scope) -> impl IntoView {
    let ssr_rates_data: Resource<(), Result<Vec<ShareGroup>, ServerFnError>> =
        create_blocking_resource(context, || (), move |_| get_rates(context));
    view! { context,
      <div>
        <Suspense fallback=|| ()>
          {move || {
              ssr_rates_data
                  .with(
                      context,
                      |ssr_rates| {
                          let mappedComponents = ssr_rates
                              .clone()
                              .unwrap()
                              .into_iter()
                              .map(|rates| {
                                  view! { context, <div>{rates.buy_price}</div> }
                              });
                          mappedComponents.collect::<Vec<HtmlElement<Div>>>()
                      },
                  )
          }}

        </Suspense>
      </div>
    }
}

