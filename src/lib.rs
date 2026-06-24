use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::Path;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Song {
    pub id: u32,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub disc: u8,
}

pub fn load_songs_from_path<P: AsRef<Path>>(path: P) -> Result<Vec<Song>, Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut songs = Vec::new();

    for record in reader.deserialize() {
        songs.push(record?);
    }

    Ok(songs)
}

pub fn draw_songs(songs: &[Song], count: usize, seed: Option<u64>) -> Vec<Song> {
    if songs.is_empty() || count == 0 {
        return Vec::new();
    }

    match seed {
        Some(value) => {
            let mut rng = StdRng::seed_from_u64(value);
            draw_songs_with_rng(songs, count, &mut rng)
        }
        None => {
            let mut rng = rand::thread_rng();
            draw_songs_with_rng(songs, count, &mut rng)
        }
    }
}

fn draw_songs_with_rng<R: rand::Rng + ?Sized>(songs: &[Song], count: usize, rng: &mut R) -> Vec<Song> {
    (0..count)
        .filter_map(|_| songs.choose(rng).cloned())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_songs() -> Vec<Song> {
        vec![
            Song {
                id: 1,
                title: "Challenger".to_string(),
                artist: "Perfume".to_string(),
                album: "Perfume The Best P Cubed".to_string(),
                disc: 1,
            },
            Song {
                id: 2,
                title: "ポリリズム".to_string(),
                artist: "Perfume".to_string(),
                album: "Perfume The Best P Cubed".to_string(),
                disc: 1,
            },
        ]
    }

    #[test]
    fn draw_songs_returns_requested_count() {
        let songs = sample_songs();

        let results = draw_songs(&songs, 5, Some(42));

        assert_eq!(results.len(), 5);
    }

    #[test]
    fn draw_songs_with_same_seed_returns_same_results() {
        let songs = sample_songs();

        let first_results = draw_songs(&songs, 5, Some(42));
        let second_results = draw_songs(&songs, 5, Some(42));

        assert_eq!(first_results, second_results);
    }

    #[test]
    fn draw_songs_returns_empty_when_count_is_zero() {
        let songs = sample_songs();

        let results = draw_songs(&songs, 0, Some(42));

        assert!(results.is_empty());
    }
}
