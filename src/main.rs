use clap::{Parser, Subcommand};
use songacha::{
    collection_progress, draw_songs, drawn_song_ranking, load_save_data_from_path,
    load_songs_from_path, missing_songs, record_pulled_songs, write_save_data_to_path,
};
use std::error::Error;
use std::fs;
use std::path::PathBuf;

const DEFAULT_SONGS_FILE: &str = "data/perfume_p_cubed.csv";
const DEFAULT_SAVE_FILE: &str = "save.json";

#[derive(Parser, Debug)]
#[command(name = "songacha")]
#[command(version)]
#[command(about = "A CLI song gacha tool using Perfume songs as sample data.")]
struct Cli {
    #[arg(long, default_value = DEFAULT_SONGS_FILE, global = true)]
    data_file: PathBuf,

    #[arg(long, default_value = DEFAULT_SAVE_FILE, global = true)]
    save_file: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Draw songs from the song list
    Pull {
        /// Number of songs to draw
        #[arg(short, long, default_value_t = 1)]
        count: usize,

        /// Random seed for reproducible results
        #[arg(short, long)]
        seed: Option<u64>,
    },

    /// Show collected songs and collection progress
    Collection {
        /// Show songs that have not been collected yet
        #[arg(long)]
        missing: bool,
    },

    /// Show frequently drawn songs
    Ranking,

    /// Reset local save data
    Reset,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Pull { count, seed } => pull_songs(&cli.data_file, &cli.save_file, count, seed)?,
        Commands::Collection { missing } => {
            show_collection(&cli.data_file, &cli.save_file, missing)?
        }
        Commands::Ranking => show_ranking(&cli.data_file, &cli.save_file)?,
        Commands::Reset => reset_save_data(&cli.save_file)?,
    }

    Ok(())
}

fn pull_songs(
    data_file: &PathBuf,
    save_file: &PathBuf,
    count: usize,
    seed: Option<u64>,
) -> Result<(), Box<dyn Error>> {
    let songs = load_songs_from_path(data_file)?;
    let mut save_data = load_save_data_from_path(save_file)?;
    let results = draw_songs(&songs, count, seed);

    record_pulled_songs(&mut save_data, &results);
    write_save_data_to_path(save_file, &save_data)?;

    println!("Gacha result");

    for song in results {
        println!("- {} / {} / Disc {}", song.title, song.artist, song.disc);
    }

    let progress = collection_progress(&songs, &save_data);
    println!(
        "Collection: {} / {}",
        progress.collected_count, progress.total_count
    );

    Ok(())
}

fn show_collection(
    data_file: &PathBuf,
    save_file: &PathBuf,
    show_missing: bool,
) -> Result<(), Box<dyn Error>> {
    let songs = load_songs_from_path(data_file)?;
    let save_data = load_save_data_from_path(save_file)?;
    let progress = collection_progress(&songs, &save_data);

    println!(
        "Collection: {} / {}",
        progress.collected_count, progress.total_count
    );

    if show_missing {
        println!("Missing songs");

        for song in missing_songs(&songs, &save_data) {
            println!("- {} / Disc {}", song.title, song.disc);
        }
    } else {
        println!("Collected songs");

        for song in songs
            .iter()
            .filter(|song| save_data.pull_counts.contains_key(&song.id))
        {
            println!("- {} / Disc {}", song.title, song.disc);
        }
    }

    Ok(())
}

fn show_ranking(data_file: &PathBuf, save_file: &PathBuf) -> Result<(), Box<dyn Error>> {
    let songs = load_songs_from_path(data_file)?;
    let save_data = load_save_data_from_path(save_file)?;
    let ranking = drawn_song_ranking(&songs, &save_data);

    if ranking.is_empty() {
        println!("No songs have been drawn yet.");
        return Ok(());
    }

    println!("Ranking");

    for (index, drawn_song) in ranking.iter().enumerate() {
        println!(
            "{}. {} - {} time(s)",
            index + 1,
            drawn_song.song.title,
            drawn_song.count
        );
    }

    Ok(())
}

fn reset_save_data(save_file: &PathBuf) -> Result<(), Box<dyn Error>> {
    if save_file.exists() {
        fs::remove_file(save_file)?;
    }

    println!("Reset local save data.");

    Ok(())
}
