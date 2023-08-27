use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Debug)]
pub struct Article {
    pub id: sqlx::types::Uuid,
    pub source_id: Option<String>,
    pub source_name: String,
    pub author: Option<String>,
    pub title: String,
    pub description: String,
    pub content: String,
    pub url: String,
    pub url_to_image: Option<String>,
    pub published_at: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewsApiPayload {
    status: NewsApiResponseStatus,
    total_results: i64,
    pub articles: Vec<NewsApiArticle>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewsApiArticle {
    pub source: Source,
    pub author: Option<String>,
    pub title: String,
    pub description: String,
    pub url: String,
    pub url_to_image: Option<String>,
    pub published_at: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Source {
    pub id: Option<String>,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
enum NewsApiResponseStatus {
    #[serde(rename = "ok")]
    Ok,
    #[serde(rename = "error")]
    Error,
}
