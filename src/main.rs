pub mod modules;

use modules::{
    url_scrape::url_scrape, 
    open_url::open_url,
    capture::capture
};


fn main() {
    let url = "https://www.example.com";
    let html_content = url_scrape(&url);
    let css_selector = "h1";
    
    open_url(&url);

    println!("{}", html_content);

    let _ = capture(&url, &css_selector);
}
