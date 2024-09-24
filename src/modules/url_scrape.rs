use reqwest;
use tokio;

#[tokio::main]
pub async fn url_scrape() -> String {
    let url = "https://example.com/";

    let response = reqwest::get(url).await;
    let html_content = response.unwrap().text().await.unwrap();
    
    html_content
}