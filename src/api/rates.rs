use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Share {
    amount: f64,
    buying_price: Option<usize>,
    company_id: String,
}

impl Share {
    fn new(amount: f64, company_id: String, buying_price: Option<usize>) -> Self {
        Share {
            amount,
            company_id,
            buying_price,
        }
    }
}

#[server(Rates, "/api")]
pub async fn rates(amount: f64) -> Result<String, ServerFnError> {
    use fantoccini::{ClientBuilder, Locator};
    use mongodb::{options::ClientOptions, Client};
    let mut options = ClientOptions::parse(dotenv!("MONGODB_CONNECTION")).await?;
    options.app_name = Some("wallet".to_string());
    let client = Client::with_options(options)?;
    let database = client.database("wallet");
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
    let collection = database.collection::<Share>("shares");
    collection
        .insert_one(Share::new(amount, "PLDINPL00011".to_string(), None), None)
        .await?;
    Ok(text)
}
