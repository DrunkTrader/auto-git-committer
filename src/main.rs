mod logic;
mod cli;
mod gui;

use cli::{CliArgs, Mode};
use clap::Parser;

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();

    match args.mode {
        Mode::Cli { repo, url, commits } => {
            use logic::{pull, update, push};
            let mut count = 1;

            if let Err(e) = pull(&repo, &url) {
                eprintln!("Pull failed: {e}");
                return;
            }

            for _ in 0..commits {
                if let Err(e) = update().and_then(|_| push(&mut count)) {
                    eprintln!("Commit failed: {e}");
                    return;
                }
            }

            println!("✅ Done! {} commits pushed to '{}'", commits, repo);
        }
        Mode::Gui => {
            if let Err(e) = gui::launch_gui() {
                eprintln!("Failed to launch GUI: {e}");
                return;
            }
            println!("✅ GUI session completed successfully");
        }
    }
}
