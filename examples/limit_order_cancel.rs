use std::env;
use indodax_rs::private::PrivateClient;
use indodax_rs::model::{TradeResponse, CancelResponse, AllOpenOrders};

fn main() {
    // Fetch API key and secret key from environment variables
    let api_key = env::var("API_KEY").expect("API_KEY not set");
    let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY not set");
    let host = "https://indodax.com".to_string(); // Replace with the actual host

    // Create a new PrivateClient
    let client = PrivateClient::new(api_key, secret_key, host);

    // Call the get_info function
    let res = client.trade(
        "eth_idr", 
        "buy", 
        Some(15_000_000), 
        Some(10_000),
        None, 
        None, 
        None,
        None,
    );

    match res {
        Ok(trade_response) => {
            let trade_res: TradeResponse = trade_response;
            println!("Trade Response: {:?}", trade_res);
            // Get Open Orders
            let open_order_res = client.get_all_open_orders();
            match open_order_res {
                Ok(open_order_response) => {
                    let open_order_res: AllOpenOrders = open_order_response;
                    println!("Open Order Response: {:?}", open_order_res);
                }
                Err(e) => {
                    eprintln!("Error fetching open order response: {:?}", e);
                }
            }
            let cleint_order_id = trade_res.client_order_id;
            // sleep for 5 seconds
            std::thread::sleep(std::time::Duration::from_secs(5));
            let cancel_res = client.cancel_order_by_client_order_id(cleint_order_id);
            match cancel_res {
                Ok(cancel_response) => {
                    let cancel_res: CancelResponse = cancel_response;
                    println!("Cancel Response: {:?}", cancel_res);
                }
                Err(e) => {
                    eprintln!("Error fetching cancel response: {:?}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Error fetching trade response: {:?}", e);
        }
    }
}