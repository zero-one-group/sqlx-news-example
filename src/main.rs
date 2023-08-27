use sqlx_news_example::http;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let response_body = http::CLIENT
        .get("https://ipinfo.io/json")
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;
    pretty_print_json(&response_body)?;
    Ok(())
}

pub fn pretty_print_json(raw_text: &str) -> Result<(), Box<dyn Error>> {
    let text: serde_json::Value = serde_json::from_str(raw_text)?;
    println!("{}", serde_json::to_string_pretty(&text)?);
    Ok(())
}
