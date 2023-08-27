use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Debug)]
pub struct Article {
    pub id: sqlx::types::Uuid,
    pub source_id: String,
    pub source_name: String,
    pub author: String,
    pub title: String,
    pub description: String,
    pub content: String,
    pub url: String,
    pub url_to_image: String,
    pub published_at: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewsApiPayload {
    status: NewsApiResponseStatus,
    total_results: i64,
    articles: Vec<NewsApiArticle>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct NewsApiArticle {
    source: Source,
    author: Option<String>,
    title: String,
    description: String,
    url: String,
    url_to_image: Option<String>,
    published_at: String,
    content: String,
}

#[derive(Serialize, Deserialize)]
pub struct Source {
    id: Option<String>,
    name: String,
}

#[derive(Serialize, Deserialize)]
enum NewsApiResponseStatus {
    #[serde(rename = "ok")]
    Ok,
    #[serde(rename = "error")]
    Error,
}
