use reqwest;
use tokio;

#[tokio::main]
pub async fn url_scrape(_url: &str, verbose: bool) -> String {
    if verbose {
        println!("Fetching HTML from {}", url);
    }

    let response = reqwest::get(_url).await;
    if verbose {
        println!("Received HTML successfully");
    }
    
    let html_content = response.unwrap().text().await.unwrap();
    
    html_content
}