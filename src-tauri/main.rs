//! Main entry point for the AIDME AI development management environment.

use dotenvy::dotenv;
mod app;
mod terminal;
mod hardware;
mod process;
mod telegram;
mod router_agent;

// Run the application
fn main() {
    dotenv().expect("Failed to load .env file. Please ensure it exists and is properly configured.");

    std::env::var("TELOXIDE_TOKEN").expect("TELOXIDE_TOKEN not set in .env file");
    std::env::var("OLLAMA_API_BASE_URL").expect("OLLAMA_API_BASE_URL not set in .env file");
    std::env::var("MY_TELEGRAM_USER_ID").expect("MY_TELEGRAM_USER_ID not set in .env file");

    app::run();
}