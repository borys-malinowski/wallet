use crate::server::api::get_rates::{get_rates, ShareGroup};
use leptos::{html::Div, *};

#[component]
pub fn Shares(context: Scope) -> impl IntoView {
    let ssr_rates_data: Resource<(), Result<Vec<ShareGroup>, ServerFnError>> =
        create_blocking_resource(context, || (), move |_| get_rates(context));
    view! { context,
      <div style="padding: 10px">
        <span>You are logged as User:</span>
      </div>
      <div style="display: flex;justify-content: center; padding: 10px;">
        <div style=" width:25%; text-alignment: center; font-weight: bold; font-size: 16px;">
          Share name
        </div>
        <div style=" width:25%; text-alignment: center; font-weight: bold; font-size: 16px;">
          Quantity
        </div>
        <div style=" width:25%; text-alignment: center; font-weight: bold; font-size: 16px;">
          Earn/Loss
        </div>
        <div style=" width:25%; text-alignment: center; font-weight: bold; font-size: 16px;">
          Value
        </div>
      </div>
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
                                  view! { context,
                                    <div style="display: flex">
                                      <table style="width:100%; border: 1px solid;border-radius: 10px;">
                                        <tr>
                                          <td style="width:25%;">{rates.share_name}</td>
                                          <td style="width:25%;">{rates.quantity}</td>
                                          <td style="width:25%;">
                                            {rates.value - (rates.buy_price * rates.quantity)}
                                          </td>
                                          <td style="width:25%;">{rates.value}</td>
                                        </tr>
                                      </table>
                                    </div>
                                  }
                              });
                          mappedComponents.collect::<Vec<HtmlElement<Div>>>()
                      },
                  )
          }}

        </Suspense>
      </div>
    }
}

