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
    .arg(
        Arg::new("verbose")
            .short('v')
            .long("verbose")
            .help("Sets the level of verbosity")
            .action(clap::ArgAction::SetTrue),
    )
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

    let verbose = matches.get_flag("verbose");
    
    if verbose {
        println!("Verbose mode enabled");
    }

    match matches.subcommand() {
        ("show_code", Some(sub_m)) => {
            let urls = sub_m.value_of("URL").unwrap();
            for url in urls {
                url_scrape(url, verbose)
            }
        }
        ("navigate", Some(sub_m)) => {
            let urls = sub_m.value_of("URL").unwrap();
            for url in urls {
                open_url(url, verbose);
            }
        }
        ("capture", Some(sub_m)) => {
            let urls = sub_m.value_of("URL").unwrap();
            let css_selector = sub_m.value_of("CSS_SELECTOR").unwrap();
            for url in urls {
                capture(url, css_selector, verbose);
            }

        }
        ("click_on", Some(sub_m)) => {
            let urls = sub_m.value_of("URL").unwrap();
            let css_selector = sub_m.value_of("CSS_SELECTOR").unwrap();
            for url in urls {
                click(url, css_selector, verbose);
            }
        }
        _ => {
            eprintln!("Invalid command. Use --help for more information.");
        }
    }
}
