use cbat_rs::{
    client::Client, product::Product, product_book::ProductBook, server_time::ServerTime,
};
use reqwest::Error;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Define the API endpoint
    //let url = "https://api.coinbase.com/api/v3/brokerage/market/products/BTC-USD";

    // Create a reqwest client
    let client = Client::new();
    /*let product = Product::get_public_product(&client, "BTC-USD").await?;
    println!("{:#?}", product);
    let server_time = ServerTime::get_public_server_time(&client).await?;
    println!("{:#?}", server_time);*/
    let product_book = ProductBook::get_public_product_book(&client, "BTC-USD", Some(10), None).await?;
    println!("{:#?}", product_book);

    // Send a GET request
    //let response = client.get(url).await?;

    // Check if the request was successful
    /*if response.status().is_success() {
        // Parse the response as JSON
        let product: Product = response.json().await?;

        // Print the deserialized JSON
            println!("{:#?}", product);
    } else {
        // Print an error message if the request failed
        eprintln!("Failed to fetch data. Status: {}", response.status());
    }*/

    Ok(())
}
