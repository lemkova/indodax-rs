use std::env;
use indodax_rs::private::PrivateClient;

fn main() {
    // Fetch API key and secret key from environment variables
    let api_key = env::var("API_KEY").expect("API_KEY not set");
    let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY not set");
    let host = "https://indodax.com".to_string(); // Replace with the actual host

    // Create a new PrivateClient
    let client = PrivateClient::new(api_key, secret_key, host);

    // Call the get_order_history function
    let orders = client.get_order_history("eth_idr", 5);
    println!("{:?}", orders);
}