use open::that;

pub fn open_url(url: &str, verbose: bool) {
    if verbose {
        println!("Navigating to {}", url);
    }

    let _= that(url);
}