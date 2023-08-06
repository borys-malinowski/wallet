use leptos::*;

#[server(Rates, "/api")]
pub async fn rates() -> Result<String, ServerFnError> {
    use fantoccini::{ClientBuilder, Locator};
    use mongodb::{options::ClientOptions, Client};
    let mut options = ClientOptions::parse(dotenv!("MONGODB_CONNECTION")).await?;
    options.app_name = Some("zsbrybnik".to_string());
    let client = Client::with_options(options).unwrap();
    println!("{:#?}", client);
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
    Ok(text)
}
