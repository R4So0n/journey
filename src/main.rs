use dotenv::dotenv;

fn main() {
    dotenv().ok();
    let api_key = std::env::var("TOKEN").expect("TOKEN not found");
    println!("API Key: {}", api_key);
}
