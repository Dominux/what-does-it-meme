use async_trait::async_trait;
use rand::Rng;
use reqwest::Client;
use scraper::{Html, Selector};

use super::interface::Scraper;
use crate::common::{
    config::Config,
    errors::{MemeError, MemeResult},
};

pub struct KnowYourMemeScraper {
    client: Client
}

const PAGE_SELECTOR: &str = "td > a.photo";

impl KnowYourMemeScraper {
    pub fn new() -> Self {
        Self {client: Client::new()}
    }

    async fn get_collection_from_page(&self) -> MemeResult<Vec<String>> {
        let (baseurl, max_pages) = {
            let config = Config::new()?;
            (config.know_your_meme_baseurl, config.know_your_meme_pages)
        };

        // Generating random page number
        let random_page = rand::thread_rng().gen_range(1..max_pages);

        // Getting memes page
        let url = format!("{}{}", baseurl, random_page);
        let response_text = self.client.get(url).send().await?.text().await?;

        // Parsing it
        let selector =
            Selector::parse(PAGE_SELECTOR).map_err(|_| MemeError::MemesScrapingError)?;
        let raw_links = Html::parse_document(response_text.as_str())
            .select(&selector)
            .filter_map(|a| a.value().attr("href").map(|l| l.to_string()))
            .collect();
        Ok(raw_links)
    }
}

#[async_trait]
impl Scraper for KnowYourMemeScraper {
    async fn get_random_memes(&self, amount: u8) -> MemeResult<Vec<String>> {
        unimplemented!()
    }

    async fn get_random_meme(&self) -> MemeResult<String> {
        unimplemented!()
    }
}
