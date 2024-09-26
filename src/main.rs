pub mod modules;

use modules::{
    capture::capture, click::click, open_url::open_url, url_scrape::url_scrape
};

use clap::Command;

fn main() {
    let matches = Command::new("ws-cli")
    .version("0.0.1")
    .author("Zoheb Khan zoheballikhan@gmail.com")
    .about("A CLI tool for web scraping")
    .subcommand(
        Command::new("show_code")
        .about("Shows the raw HTML code of a URL")
        .arg(Arg::with_name("URL").required(true).takes_value(true)),
    )
    .subcommand(
        Command::new("navigate")
        .about("Navigates to a URL")
        .arg(Arg::with_name("URL").required(true).takes_value(true)),
    )
    .subcommand(
        Command::new("capture")
        .about("Captures a subsection of the HTML using a CSS selector")
        .arg(Arg::with_name("URL").required(true).takes_value(true))
        .arg(Arg::with_name("CSS_SELECTOR").required(true).takes_value(true)),
    )
    .subcommand(
        Command::new("click_on")
        .about("Clicks on an element using a CSS selector")
        .arg(Arg::with_name("URL").required(true).takes_value(true))
        .arg(Arg::with_name("CSS_SELECTOR").required(true).takes_value(true)),
    )
    .get_matches();

    match matches.subcommand() {
        ("show_code", Some(sub_m)) => {
            let url = sub_m.value_of("URL").unwrap();
            url_scrape(url);
        }
        ("navigate", Some(sub_m)) => {
            let url = sub_m.value_of("URL").unwrap();
            open_url(url);
        }
        ("capture", Some(sub_m)) => {
            let url = sub_m.value_of("URL").unwrap();
            let css_selector = sub_m.value_of("CSS_SELECTOR").unwrap();
            capture(url, css_selector);
        }
        ("click_on", Some(sub_m)) => {
            let url = sub_m.value_of("URL").unwrap();
            let css_selector = sub_m.value_of("CSS_SELECTOR").unwrap();
            click(url, css_selector);
        }
        _ => {
            eprintln!("Invalid command. Use --help for more information.");
        }
    }
}
