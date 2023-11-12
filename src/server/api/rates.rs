use leptos::*;

#[server(Rates, "/api")]
pub async fn rates(isin: String, quantity: f32) -> Result<String, ServerFnError> {
    use fantoccini::{ClientBuilder, Locator};
    use prisma_cli::prisma::PrismaClient;
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
    let client = PrismaClient::_builder().build().await?;
    client
        .market_transaction()
        .create(
            isin,
            String::from("share_name"),
            quantity.into(),
            2137.0,
            vec![],
        )
        .exec()
        .await?;
    // let trasactionss = client.market_transaction().find_many(vec![]).exec().await?;
    // println!("{:#?}", trasactionss);
    Ok(text)
}




