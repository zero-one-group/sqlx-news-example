use sqlx::postgres::PgPoolOptions;
use sqlx_news_example::{env, http, news};
use std::error::Error;

type Pool = sqlx::Pool<sqlx::Postgres>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv()?;
    let database_url = env::load_env("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    print_all_articles(&pool).await?;

    Ok(())
}

pub async fn print_all_articles(pool: &Pool) -> Result<(), Box<dyn Error>> {
    let articles = sqlx::query_as!(news::Article, " SELECT * FROM articles ")
        .fetch_all(pool) // -> Vec<Country>
        .await?;
    println!("{:#?}", articles);
    Ok(())
}

pub async fn print_ipinfo() -> Result<(), Box<dyn Error>> {
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
