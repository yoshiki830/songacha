use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "songacha")]
#[command(version)]
#[command(about = "A CLI song gacha tool using Perfume songs as sample data.")]
struct Cli {
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
    let cli = Cli::parse();

    match cli.command {
        Commands::Pull { count, seed } => {
            println!("Draw {} song(s).", count);

            if let Some(seed) = seed {
                println!("Seed: {}", seed);
            }
        }
        Commands::Collection { missing } => {
            if missing {
                println!("Show missing songs.");
            } else {
                println!("Show collection progress.");
            }
        }
        Commands::Ranking => {
            println!("Show frequently drawn songs ranking.");
        }
        Commands::Reset => {
            println!("Reset local save data.");
        }
    }
}
