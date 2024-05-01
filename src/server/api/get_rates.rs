use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShareGroup {
    pub buy_price: f64,
    pub quantity: f64,
    pub share_name: String,
    pub value: f64,
}

#[server(GetRates, "/api")]
pub async fn get_rates(context: Scope) -> Result<Vec<ShareGroup>, ServerFnError> {
    use crate::server::clients::postgre_sql_client::use_postgre_sql_client;
    let client = use_postgre_sql_client(context).unwrap();
    let transactions = client.market_transaction().find_many(vec![]).exec().await?;
    let mapped_transactions: Vec<ShareGroup> = transactions.into_iter().map(|transaction| {
        ShareGroup {
            buy_price: transaction.buy_price,
            quantity: transaction.quantity,
            share_name: transaction.share_name,
            value: transaction.value,
        }
    }).collect();
    Ok(mapped_transactions)
}










































































































