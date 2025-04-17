use serde::Deserialize;

#[derive(Deserialize)]
pub struct Search {
    pub id: u64,
    pub title: String,
    pub original_language: String,
    pub original_title: String,
    pub overview: String,
    pub poster_path: String,
    pub media_type: String,
    pub genre_ids: Vec<i16>,
    pub release_date: String,
    pub vote_average: f32,
    pub vote_count: i32,
}