use crate::apps::memes::scrapers::get_scrapers;
use crate::common::config::Config;
use crate::common::db::DBConnection;
use crate::common::errors::MemeResult;

use super::models;
use super::repository::MemesRepository;
use super::scrapers::interface::Scraper;

pub struct MemesService<'a> {
    repo: MemesRepository<'a>,
}

impl<'a> MemesService<'a> {
    pub fn new(db: &'a DBConnection) -> Self {
        Self {
            repo: MemesRepository::new(db),
        }
    }

    pub fn save_meme(&self, meme: models::Meme) -> MemeResult<()> {
        self.repo.save_meme_if_not_exists(meme)
    }

    // pub fn get_meme(&self, meme_id: uuid::Uuid) -> MemeResult<models::Meme> {
    //     self.repo.get_meme(meme_id)
    // }

    pub fn count_memes(&self, round_id: uuid::Uuid) -> MemeResult<u8> {
        self.repo.memes_count(round_id)
    }

    pub fn save_voter(&self, meme_id: uuid::Uuid, voter_id: uuid::Uuid) -> MemeResult<()> {
        let mut voters_ids = self.repo.get_meme(meme_id)?.voters_ids;

        // if voter is already is in the list of the meme's voters -> skip his saving
        if !voters_ids.contains(&voter_id) {
            voters_ids.push(voter_id);
            self.repo.update_voters_ids(meme_id, voters_ids)?
        }

        Ok(())
    }

    pub async fn get_random_memes() -> MemeResult<Vec<String>> {
        let scrapers = get_scrapers();

        // TODO: implement logic to distribute load among any amount of scrapers
        // Getting random memes
        let memes = scrapers[0]
            .get_random_memes(Config::new()?.memes_in_hand_amount)
            .await?;
        Ok(memes)
    }

    pub async fn get_random_meme() -> MemeResult<String> {
        let scrapers = get_scrapers();

        // TODO: implement logic to distribute load among any amount of scrapers
        // Getting random memes
        scrapers[0].get_random_meme().await
        
    }
}
