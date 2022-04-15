use async_trait::async_trait;

use super::interface::Scraper;
use crate::common::{errors::MemeResult, config::Config};

pub struct KnowYourMemeScraper {}

impl KnowYourMemeScraper {
    async fn get_page() -> MemeResult<Vec<String>> {
        let client = reqwest::Client::new();
        let (baseurl, max_pages) = {
            let config = Config::new()?;
            (config.know_your_meme_baseurl, config.know_your_meme_pages)
        };

        unimplemented!()
    }
}

#[async_trait]
impl Scraper for KnowYourMemeScraper {
    async fn get_random_memes(amount: u8) -> MemeResult<Vec<String>> {
        unimplemented!()
    }

    async fn get_random_meme() -> MemeResult<String> {
        unimplemented!()
    }
}
