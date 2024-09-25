use std::io::Error;
use scraper::{Html, Selector};

use super::{
    url_scrape::url_scrape, 
    click::click
};

pub fn capture(url: &str, css_selector: &str) -> Result<(), Error> {
    let html_content = url_scrape(url);
    let document = Html::parse_document(&html_content);

    let selector = Selector::parse(css_selector).unwrap();

    println!("{} selectors", &css_selector);
    for element in document.select(&selector) {
        println!("{}", element.inner_html());
    }

    

    Ok(())
}