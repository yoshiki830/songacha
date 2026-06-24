//! Core functions for loading songs, drawing songs, and managing collection data.

use rand::SeedableRng;

use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::path::Path;

/// A song record loaded from the local CSV file.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Song {
    pub id: u32,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub disc: u8,
}

/// Local save data that stores how many times each song has been drawn.
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq)]
pub struct SaveData {
    pub pull_counts: HashMap<u32, u32>,
}

/// Summary of the user's collection progress.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CollectionProgress {
    pub collected_count: usize,
    pub total_count: usize,
}

/// A song with its draw count, used for ranking output.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DrawnSong {
    pub song: Song,
    pub count: u32,
}

/// Loads song records from a CSV file.
pub fn load_songs_from_path<P: AsRef<Path>>(path: P) -> Result<Vec<Song>, Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut songs = Vec::new();

    for record in reader.deserialize() {
        songs.push(record?);
    }

    Ok(songs)
}

/// Loads save data from a JSON file.
pub fn load_save_data_from_path<P: AsRef<Path>>(path: P) -> Result<SaveData, Box<dyn Error>> {
    let path = path.as_ref();

    if !path.exists() {
        return Ok(SaveData::default());
    }

    let text = fs::read_to_string(path)?;

    if text.trim().is_empty() {
        return Ok(SaveData::default());
    }

    Ok(serde_json::from_str(&text)?)
}

/// Writes save data to a JSON file.
pub fn write_save_data_to_path<P: AsRef<Path>>(
    path: P,
    save_data: &SaveData,
) -> Result<(), Box<dyn Error>> {
    let path = path.as_ref();

    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        fs::create_dir_all(parent)?;
    }

    let text = serde_json::to_string_pretty(save_data)?;
    fs::write(path, text)?;

    Ok(())
}

/// Draws songs randomly from the given song list.
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

/// Updates save data using the songs drawn by the user.
pub fn record_pulled_songs(save_data: &mut SaveData, songs: &[Song]) {
    for song in songs {
        *save_data.pull_counts.entry(song.id).or_insert(0) += 1;
    }
}

/// Calculates how many songs have been collected.
pub fn collection_progress(songs: &[Song], save_data: &SaveData) -> CollectionProgress {
    let collected_ids: HashSet<u32> = save_data.pull_counts.keys().copied().collect();

    CollectionProgress {
        collected_count: songs
            .iter()
            .filter(|song| collected_ids.contains(&song.id))
            .count(),
        total_count: songs.len(),
    }
}

/// Returns songs that have not been collected yet.
pub fn missing_songs(songs: &[Song], save_data: &SaveData) -> Vec<Song> {
    let collected_ids: HashSet<u32> = save_data.pull_counts.keys().copied().collect();

    songs
        .iter()
        .filter(|song| !collected_ids.contains(&song.id))
        .cloned()
        .collect()
}

/// Returns drawn songs sorted by draw count.
pub fn drawn_song_ranking(songs: &[Song], save_data: &SaveData) -> Vec<DrawnSong> {
    let mut ranking: Vec<DrawnSong> = songs
        .iter()
        .filter_map(|song| {
            save_data.pull_counts.get(&song.id).map(|count| DrawnSong {
                song: song.clone(),
                count: *count,
            })
        })
        .collect();

    ranking.sort_by(|left, right| {
        right
            .count
            .cmp(&left.count)
            .then_with(|| left.song.id.cmp(&right.song.id))
    });

    ranking
}

fn draw_songs_with_rng<R: rand::Rng + ?Sized>(
    songs: &[Song],
    count: usize,
    rng: &mut R,
) -> Vec<Song> {
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

    #[test]
    fn record_pulled_songs_updates_pull_counts() {
        let songs = sample_songs();
        let mut save_data = SaveData::default();

        record_pulled_songs(&mut save_data, &songs);

        assert_eq!(save_data.pull_counts.get(&1), Some(&1));
        assert_eq!(save_data.pull_counts.get(&2), Some(&1));
    }

    #[test]
    fn collection_progress_counts_collected_songs() {
        let songs = sample_songs();
        let mut save_data = SaveData::default();
        save_data.pull_counts.insert(1, 3);

        let progress = collection_progress(&songs, &save_data);

        assert_eq!(progress.collected_count, 1);
        assert_eq!(progress.total_count, 2);
    }

    #[test]
    fn drawn_song_ranking_sorts_by_count() {
        let songs = sample_songs();
        let mut save_data = SaveData::default();
        save_data.pull_counts.insert(1, 1);
        save_data.pull_counts.insert(2, 3);

        let ranking = drawn_song_ranking(&songs, &save_data);

        assert_eq!(ranking[0].song.id, 2);
        assert_eq!(ranking[0].count, 3);
    }
}
