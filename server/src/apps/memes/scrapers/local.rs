use std::{
    fs::File,
    io::{self, BufRead},
};

use async_trait::async_trait;
use rand::prelude::IteratorRandom;

use super::interface::Scraper;
use crate::common::errors::{MemeError, MemeResult};

pub struct LocalScrapper<'a> {
    path: &'a str,
}

impl<'a> LocalScrapper<'a> {
    pub fn new(path: &'a str) -> Self {
        Self { path }
    }

    fn get_lines(&self) -> MemeResult<io::Lines<io::BufReader<File>>> {
        let file = File::open(self.path).map_err(|_| MemeError::MemesScrapingError)?;
        Ok(io::BufReader::new(file).lines())
    }
}

#[async_trait]
impl<'a> Scraper<'a> for LocalScrapper<'a> {
    async fn get_random_memes(&self, amount: u8) -> MemeResult<Vec<String>> {
        self.get_lines()?
            .choose_multiple(&mut rand::thread_rng(), amount as usize)
            .into_iter()
            .map(|meme| meme.map_err(|_| MemeError::MemesScrapingError))
            .collect::<MemeResult<Vec<_>>>()
    }

    async fn get_random_meme(&self) -> MemeResult<String> {
        self.get_lines()?
            .choose(&mut rand::thread_rng())
            .ok_or(MemeError::MemesScrapingError)?
            .map_err(|_| MemeError::MemesScrapingError)
    }
}
