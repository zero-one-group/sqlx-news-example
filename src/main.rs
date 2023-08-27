// TODO: command-line app to dump to database
use clap::{Parser, ValueEnum};
use sqlx::postgres::PgPoolOptions;
use sqlx_news_example::{env, http, news, types};
use std::error::Error;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long)]
    app: App,
    #[clap(short, long, default_value_t = 3)]
    limit: i64,
}

#[derive(ValueEnum, Debug, Clone)]
enum App {
    HealthCheck,
    SeedArticles,
    PrintArticles,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv()?;
    let articles = news::get_last_week_articles("dogecoin").await?;
    println!("{:#?}", articles);
    println!("Fetched {} articles!", articles.len());

    let args = Args::parse();
    let pool = initialise_db_pool().await?;
    match args.app {
        App::HealthCheck => print_ipinfo().await?,
        App::SeedArticles => seed_articles(&pool).await?,
        App::PrintArticles => print_articles(&pool, args.limit).await?,
    }
    Ok(())
}

pub async fn initialise_db_pool() -> Result<types::Pool, Box<dyn Error>> {
    let database_url = env::load_env("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    Ok(pool)
}

pub async fn seed_articles(pool: &types::Pool) -> Result<(), Box<dyn Error>> {
    let contents = std::fs::read_to_string("resources/seed_articles.json")?;
    let payload: news::NewsApiPayload = serde_json::from_str(&contents)?;

    for article in payload.articles {
        let maybe_inserted = news::insert_article(pool, &article).await?;
        match maybe_inserted {
            Some(inserted) => println!("Inserted: {}", inserted.url),
            None => println!("Skipping: {}", article.url),
        }
        println!("___________________________________________________");
    }

    Ok(())
}

pub async fn print_articles(pool: &types::Pool, limit: i64) -> Result<(), Box<dyn Error>> {
    let articles = sqlx::query_as!(news::Article, " SELECT * FROM articles LIMIT $1", limit)
        .fetch_all(pool)
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
