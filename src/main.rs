use clap::{Parser, Subcommand};

mod analyzer;

#[derive(Parser)]
#[command(name = "gitgrade")]
#[command(about = "Automatic git activity analyzer for students")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan a git repository and show your activity
    Scan {
        /// Path to the git repo (default: current directory)
        #[arg(default_value = ".")]
        path: String,
    },
    /// Show your coding streak
    Streak {
        #[arg(default_value = ".")]
        path: String,
    },
    /// Show commits per day this week
    Weekly {
        #[arg(default_value = ".")]
        path: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan { path } => analyzer::scan(&path),
        Commands::Streak { path } => analyzer::streak(&path),
        Commands::Weekly { path } => analyzer::weekly(&path),
    }
}
