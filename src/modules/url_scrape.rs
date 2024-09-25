use reqwest;
use tokio;

#[tokio::main]
pub async fn url_scrape(_url: &str) -> String {
    let response = reqwest::get(_url).await;
    let html_content = response.unwrap().text().await.unwrap();
    
    html_content
}