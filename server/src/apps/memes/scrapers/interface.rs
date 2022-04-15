use async_trait::async_trait;

use crate::common::errors::MemeResult;

#[async_trait]
pub trait Scraper {
    async fn get_random_memes(&self, amount: u8) -> MemeResult<Vec<String>>;
    async fn get_random_meme(&self) -> MemeResult<String>;
}
