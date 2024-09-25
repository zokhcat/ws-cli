pub mod modules;

use modules::{ url_scrape::url_scrape, open_url::open_url};


fn main() {
    let site = "https://www.reddit.com";

    let html_content = url_scrape();
    open_url(&site);

    println!("{}", html_content);
}
