use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "Git Automator")]
#[command(about = "Automate git workflow via CLI or GUI", long_about = None)]
pub struct CliArgs {
    #[command(subcommand)]
    pub mode: Mode,
}

#[derive(Subcommand, Debug)]
pub enum Mode {
    /// Run in CLI mode
    Cli {
        #[arg(short, long)]
        repo: String,

        #[arg(short, long)]
        url: String,

        #[arg(short, long, default_value_t = 5)]
        commits: i32,
    },
    
    /// Launch the GUI
    Gui,
}
