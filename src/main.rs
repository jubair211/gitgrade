use clap::{Parser, Subcommand};

mod analyzer;

#[derive(Parser)]
#[command(name = "gitgrade")]
#[command(about = "Coding habit tracker for 1st and 2nd semester Health Informatics students")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan a git repository and show your activity
    Scan {
        #[arg(default_value = ".")]
        path: String,
    },
    /// Show your coding habits and consistency
    Habits {
        #[arg(default_value = ".")]
        path: String,
    },
    /// Show your progress since week 1
    Progress {
        #[arg(default_value = ".")]
        path: String,
    },
    /// Show your coding time patterns
    Patterns {
        #[arg(default_value = ".")]
        path: String,
    },
    /// Show your beginner milestones
    Milestones {
        #[arg(default_value = ".")]
        path: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan { path } => analyzer::scan(&path),
        Commands::Habits { path } => analyzer::habits(&path),
        Commands::Progress { path } => analyzer::progress(&path),
        Commands::Patterns { path } => analyzer::patterns(&path),
        Commands::Milestones { path } => analyzer::milestones(&path),
    }
}
