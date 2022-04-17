use crate::apps::memes::scrapers::get_scrapers;
use crate::common::config::Config;
use crate::common::errors::MemeResult;

use super::scrapers::interface::Scraper;

pub struct MemesService {}

impl MemesService {
    pub async fn get_random_memes() -> MemeResult<Vec<String>> {
        let scrapers = get_scrapers();

        // TODO: implement logic to distribute load among any amount of scrapers
        // Getting random memes
        let memes = scrapers[0]
            .get_random_memes(Config::new()?.memes_in_hand_amount)
            .await?;
        Ok(memes)
    }
}
