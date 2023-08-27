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

    seed_articles(&pool).await?;
    print_all_articles(&pool).await?;

    // TODO: use Clap to do health check, seed, print 10 articles.
    // TODO: lazy_static init connection pool.

    Ok(())
}

pub async fn seed_articles(pool: &Pool) -> Result<(), Box<dyn Error>> {
    let contents = std::fs::read_to_string("resources/seed_articles.json")?;
    let payload: news::NewsApiPayload = serde_json::from_str(&contents)?;

    for article in payload.articles {
        let maybe_inserted = sqlx::query_as!(
            news::Article,
            r#"
        INSERT INTO articles(source_id, source_name, author, title, description,
        content, url, url_to_image, published_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        ON CONFLICT (url) DO NOTHING RETURNING *;
        "#,
            article.source.id,
            article.source.name,
            article.author,
            article.title,
            article.description,
            article.content,
            article.url,
            article.url_to_image,
            chrono::DateTime::parse_from_rfc3339(&article.published_at)?.timestamp_millis(),
        )
        .fetch_optional(pool)
        .await?;
        match maybe_inserted {
            Some(inserted) => println!("Inserted: {}", inserted.url),
            None => println!("Skipping: {}", article.url),
        }
        println!("___________________________________________________");
    }

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
