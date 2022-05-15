pub mod interface;
pub mod local;
// pub mod know_your_meme;

use interface::Scraper;

use crate::common::config::Config;

pub fn get_scrapers(config: &Config) -> Vec<impl Scraper> {
    // vec![know_your_meme::KnowYourMemeScraper::new()]
    vec![local::LocalScrapper::new(config.local_memes_path.as_str())]
}
