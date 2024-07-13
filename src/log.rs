use serde_json::json;
use reqwest::Client;
use std::error::Error;
use anyhow::Result;
use std::env::var;
use dotenv::dotenv;


pub async fn discord_log_webhook(
    log_message: &str
) -> Result<(), Box<dyn Error>> {
    let client: Client = Client::new();
    dotenv().ok();
    
    let webhook_url: String = match var("WATCHDOG_RS_DISCORD_WEBHOOK") {
        Ok(url) => url,
        Err(_) => return Err("WATCHDOG_RS_DISCORD_WEBHOOK must be set".into()),
    };

    let formatted_log_message: String = format!(
        "```watchdog_rs | {}```", log_message
    );

    println!("\x1b[34;1m{}\x1b[0m", formatted_log_message);

    client.post(&webhook_url)
        .body(json!({
            "content": formatted_log_message
        }).to_string())

        .header(
            "Content-Type","application/json"
        )
        .send()
        .await?;

    Ok(())

}