use std::env;
use indodax_rs::private::PrivateClient;
use indodax_rs::model::AccountInfo;

fn main() {
    // Fetch API key and secret key from environment variables
    let api_key = env::var("API_KEY").expect("API_KEY not set");
    let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY not set");
    let host = "https://indodax.com".to_string(); // Replace with the actual host

    // Create a new PrivateClient
    let client = PrivateClient::new(api_key, secret_key, host);

    // Call the get_info function
    match client.get_info() {
        Ok(account_info) => {
            println!("Account Info: {:?}", account_info);
            // User id, user name
            println!("User ID: {}", account_info.user_id);
            println!("User Name: {}", account_info.name);
        }
        Err(e) => {
            eprintln!("Error fetching account info: {:?}", e);
        }
    }
}