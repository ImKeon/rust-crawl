use arr_macro::arr;
use reqwest::header::USER_AGENT;
use scraper::{Html, Selector};
use reqwest;
use tokio;

#[cfg(test)]
mod crawl {
    use std::fmt::format;
    use reqwest::header::USER_AGENT;
    use scraper::{Html, Selector};
    use serde::de::Error;

    #[tokio::test]
    pub async fn crawl() -> Result<(), reqwest::Error> {
        let url = "https://sports.news.naver.com/kfootball/record/index?category=kleague&year=2023";
        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()?;
        let res = client.get(url)
            .header(USER_AGENT, "User-Agent=Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.114 Safari/537.36")
            .send().await?;
        let body = res.text().await?;

        println!("Body:\n\n{:?}", body);

        Ok(())
    }
}