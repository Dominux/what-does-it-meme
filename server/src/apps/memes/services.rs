use crate::common::errors::MemeResult;

pub struct MemesService {}

impl MemesService {
    pub async fn get_random_memes() -> MemeResult<Vec<String>> {


        Ok(vec!())
    }
}
