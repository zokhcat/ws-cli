use thirtyfour::prelude::*;

#[tokio::main]
pub async fn click(url: &str, css_selector: &str) -> WebDriverResult<()> {
    let caps = DesiredCapabilities::firefox();
    let driver = WebDriver::new("http://localhost:4444", caps).await?;

    driver.goto(url);
    driver.wait_for_element(By::Css(css_selector)).await?;

    driver.find_element(By::Css(css_selector)).await?.click().await?;

    driver.quit().await?;

    Ok(())
}