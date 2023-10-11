use leptos::*;
use serde::{Deserialize, Serialize};

const DATABASE_URL: &str = "mysql://root:password@localhost:3306";
const DB_NAME: &str = "bakeries_db";

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Share {
    amount: f32,
    buying_price: Option<usize>,
    isin: String,
}

impl Share {
    fn new(amount: f32, isin: String, buying_price: Option<usize>) -> Self {
        Share {
            amount,
            isin,
            buying_price,
        }
    }
}

#[server(Rates, "/api")]
pub async fn rates(isin: String) -> Result<String, ServerFnError> {
    use fantoccini::{ClientBuilder, Locator};
    use prisma_client_rust::NewClientError;
    // use sea_orm::{Database, DatabaseConnection};
    // let db: DatabaseConnection = Database::connect(dotenv!("POSTGRES_CONNECTION")).await?;
    // use mongodb::{options::ClientOptions, Client};
    // let mut options = ClientOptions::parse(dotenv!("MONGODB_CONNECTION")).await?;
    // options.app_name = Some("wallet".to_string());
    // let clientMongo = Client::with_options(options)?;
    // let database = client.database("wallet");
    let connection = ClientBuilder::native()
        .connect("http://localhost:4444")
        .await
        .expect("failed to connect to WebDriver");
    connection
        .goto(&format!("https://www.gpw.pl/spolka?isin={isin}"))
        .await?;
    let text = connection
        .find(Locator::Css(".summary"))
        .await?
        .text()
        .await?;
    let client: Result<PrismaClient, NewClientError> = PrismaClient::_builder().build().await;
    // let collection = database.collection::<Share>("shares");
    // collection
    //     .insert_one(Share::new(amount, "PLDINPL00011".to_string(), None), None)
    //     .await?;
    Ok(text)
}
