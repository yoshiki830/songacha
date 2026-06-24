use clap::{Parser, Subcommand};
use songacha::{draw_songs, load_songs_from_path};
use std::error::Error;
use std::path::PathBuf;

const DEFAULT_SONGS_FILE: &str = "data/perfume_p_cubed.csv";

#[derive(Parser, Debug)]
#[command(name = "songacha")]
#[command(version)]
#[command(about = "A CLI song gacha tool using Perfume songs as sample data.")]
struct Cli {
    #[arg(short, long, default_value = DEFAULT_SONGS_FILE)]
    data_file: PathBuf,

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
        Commands::Pull { count, seed } => pull_songs(&cli.data_file, count, seed)?,
        Commands::Collection { missing } => show_collection(missing),
        Commands::Ranking => show_ranking(),
        Commands::Reset => reset_save_data(),
    }

    Ok(())
}

fn pull_songs(data_file: &PathBuf, count: usize, seed: Option<u64>) -> Result<(), Box<dyn Error>> {
    let songs = load_songs_from_path(data_file)?;
    let results = draw_songs(&songs, count, seed);

    println!("Gacha result");

    for song in results {
        println!("- {} / {} / Disc {}", song.title, song.artist, song.disc);
    }

    Ok(())
}

fn show_collection(missing: bool) {
    if missing {
        println!("Show missing songs.");
    } else {
        println!("Show collection progress.");
    }
}

fn show_ranking() {
    println!("Show frequently drawn songs ranking.");
}

fn reset_save_data() {
    println!("Reset local save data.");
}
