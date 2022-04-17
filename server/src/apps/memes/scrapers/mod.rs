pub mod interface;
pub mod know_your_meme;

use interface::Scraper;

pub fn get_scrapers() -> Vec<impl Scraper> {
    vec![know_your_meme::KnowYourMemeScraper::new()]
}
