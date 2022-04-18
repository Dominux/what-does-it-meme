use async_trait::async_trait;
use futures::future::join_all;
use rand::{prelude::SliceRandom, Rng};
use reqwest::Client;
use scraper::{Html, Selector};

use super::interface::Scraper;
use crate::common::{
    config::Config,
    errors::{MemeError, MemeResult},
};

pub struct KnowYourMemeScraper {
    client: Client,
    config: Config,
}

const PAGE_SELECTOR: &str = "td > a.photo";
const MEME_SELECTOR: &str = "a.photo > img.wide";

impl KnowYourMemeScraper {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            config: Config::new().expect("Error with config initialization"),
        }
    }

    async fn get_raw_links_from_random_page(&self) -> MemeResult<Vec<String>> {
        // Generating random page number
        let random_page = rand::thread_rng().gen_range(1..self.config.know_your_meme_pages);

        // Getting memes page
        let url = format!("{}{}", self.config.know_your_meme_pageurl, random_page);
        let response_text = self
            .client
            .get(&url)
            .header("User-Agent", "ur mom")
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;

        // Parsing it
        let selector = Selector::parse(PAGE_SELECTOR).map_err(|_| MemeError::MemesScrapingError)?;
        let raw_links = Html::parse_document(response_text.as_str())
            .select(&selector)
            // TODO: raw links are relative, create absolute ones from them
            .filter_map(|a| a.value().attr("href").map(|l| l.to_string()))
            .collect();

        Ok(raw_links)
    }

    async fn get_meme_by_raw_link(&self, raw_link: String) -> MemeResult<String> {
        let response_text = self
            .client
            .get(raw_link)
            .header("User-Agent", "ur mom")
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;

        let selector = Selector::parse(MEME_SELECTOR).map_err(|_| MemeError::MemesScrapingError)?;
        let meme_link = Html::parse_document(response_text.as_str())
            .select(&selector)
            .filter_map(|img| img.value().attr("src").map(|l| l.to_string()))
            .next()
            .ok_or(MemeError::MemesScrapingError)?;
        Ok(meme_link)
    }
}

#[async_trait]
impl Scraper for KnowYourMemeScraper {
    async fn get_random_memes(&self, amount: u8) -> MemeResult<Vec<String>> {
        // Getting raw memes, choosing multiple unrepeated ones and getting vector of futures
        let memes_futures: Vec<_> = self
            .get_raw_links_from_random_page()
            .await?
            .choose_multiple(&mut rand::thread_rng(), amount as usize)
            .cloned()
            .map(|raw_link| self.get_meme_by_raw_link(raw_link))
            .collect();

        // Running reqeusts concurrently and raising an error if any request has failed
        let memes_results = join_all(memes_futures).await;
        let mut memes: Vec<String> = vec![];
        for result in memes_results {
            memes.push(result?)
        }

        Ok(memes)
    }

    async fn get_random_meme(&self) -> MemeResult<String> {
        // Getting raw links to memes and choosing one of them randomly
        let raw_link = self
            .get_raw_links_from_random_page()
            .await?
            .choose(&mut rand::thread_rng())
            .ok_or(MemeError::MemesScrapingError)?
            .to_string();

        // Retreiving the meme link itself
        let meme = self.get_meme_by_raw_link(raw_link).await?;
        Ok(meme)
    }
}
