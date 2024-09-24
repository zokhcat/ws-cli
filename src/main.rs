pub mod modules;

use crate::modules::url_scrape::url_scrape;

fn main() {

    let html_content = url_scrape();

    println!("{}", html_content);
}
