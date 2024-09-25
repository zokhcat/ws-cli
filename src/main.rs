pub mod modules;

use modules::{
    capture::capture, click::click, open_url::open_url, url_scrape::url_scrape
};

fn main() {
    let url = "https://www.example.com";
    let html_content = url_scrape(&url);
    let css_selector = "a";
    
    open_url(&url);

    println!("{}", html_content);

    let _ = capture(&url, &css_selector);
    let _ = click(&url, &css_selector);
}
