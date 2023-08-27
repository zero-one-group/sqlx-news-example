pub mod domain;

pub use crate::{env, http, types};
pub use domain::{Article, NewsApiArticle, NewsApiPayload};
use std::error::Error;

const NEWS_API_EVERYTHING_URL: &str = "https://newsapi.org/v2/everything";
const DATE_FORMAT: &str = "%Y-%m-%d";

pub async fn fetch_last_week_articles(
    query: &str,
    page: usize,
) -> Result<NewsApiPayload, Box<dyn Error>> {
    let api_token = env::load_env("NEWS_API_TOKEN")?;
    let today = chrono::Utc::now();
    let from_date = today.format(DATE_FORMAT).to_string();
    let last_week = today - chrono::Duration::days(7);
    let to_date = last_week.format(DATE_FORMAT).to_string();

    let payload = http::CLIENT
        .get(NEWS_API_EVERYTHING_URL)
        .query(&[
            ("q", query),
            ("apiKey", &api_token),
            ("from", &from_date),
            ("to", &to_date),
            ("page", &page.to_string()),
        ])
        .header("User-Agent", http::APP_USER_AGENT)
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;
    let payload: NewsApiPayload = serde_json::from_str(&payload)?;
    Ok(payload)
}

pub async fn insert_article(
    pool: &types::Pool,
    article: &NewsApiArticle,
) -> Result<Option<Article>, Box<dyn Error>> {
    let inserted = sqlx::query_as!(
        Article,
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

    Ok(inserted)
}
