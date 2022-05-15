use crate::apps::memes::scrapers::get_scrapers;
use crate::common::config::Config;
use crate::common::db::DBConnection;
use crate::common::errors::{MemeError, MemeResult};

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

    pub fn save_meme_if_not_exists(&self, meme: models::Meme) -> MemeResult<bool> {
        self.repo.save_meme_if_not_exists(meme)
    }

    pub fn get_meme(&self, meme_id: uuid::Uuid) -> MemeResult<models::Meme> {
        self.repo.get_meme(meme_id)
    }

    pub fn count_memes(&self, round_id: uuid::Uuid) -> MemeResult<u8> {
        self.repo.memes_count(round_id)
    }

    pub fn list_memes(&self, round_id: uuid::Uuid) -> MemeResult<Vec<models::Meme>> {
        self.repo.list_memes(round_id)
    }

    pub fn list_memes_by_rounds_ids(
        &self,
        rounds_ids: Vec<uuid::Uuid>,
    ) -> MemeResult<Vec<models::Meme>> {
        self.repo.list_memes_by_rounds_ids(rounds_ids)
    }

    pub fn list_all_round_voters_ids(&self, round_id: uuid::Uuid) -> MemeResult<Vec<uuid::Uuid>> {
        self.repo.list_all_round_voters_ids(round_id)
    }

    pub fn save_voter(&self, meme_id: uuid::Uuid, voter_id: uuid::Uuid) -> MemeResult<()> {
        let mut meme = self.repo.get_meme(meme_id)?;

        // Validating if voter has already voted in this round
        let all_round_voters_ids = self.repo.list_all_round_voters_ids(meme.round_id)?;
        if all_round_voters_ids.contains(&voter_id) {
            return Err(MemeError::AlreadyVoted);
        }

        meme.voters_ids.push(voter_id);
        self.repo.update_voters_ids(meme_id, meme.voters_ids)
    }

    ////////////////////////////////////////////////////////////////////////////////////////////
    //  Getting memes
    ////////////////////////////////////////////////////////////////////////////////////////////

    pub async fn get_random_memes(config: &Config) -> MemeResult<Vec<String>> {
        let scrapers = get_scrapers(config);

        // TODO: implement logic to distribute load among any amount of scrapers
        // Getting random memes
        let memes = scrapers[0]
            .get_random_memes(Config::new()?.memes_in_hand_amount)
            .await?;
        Ok(memes)
    }

    pub async fn get_random_meme(config: &Config) -> MemeResult<String> {
        let scrapers = get_scrapers(config);

        // TODO: implement logic to distribute load among any amount of scrapers
        // Getting random memes
        scrapers[0].get_random_meme().await
    }
}
