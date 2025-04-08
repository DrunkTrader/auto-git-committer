use dialoguer::{Input, theme::ColorfulTheme};
use crossterm::style::{Stylize, Print};
use std::io::{self, stdout};
use crossterm::execute;

pub struct UserInput {
    pub repo: String,
    pub url: String,
    pub commits: i32,
}

pub fn collect_user_input() -> io::Result<UserInput> {
    execute!(
        stdout(),
        Print("\n🚀 ".green().bold()),
        Print(" Git Automation UI Tool\n\n")
    )?;

    let theme = ColorfulTheme::default();

    let repo: String = Input::with_theme(&theme)
        .with_prompt("Enter Git Repository name")
        .interact_text().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    let url: String = Input::with_theme(&theme)
        .with_prompt("Enter Git Repository URL")
        .interact_text().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    let commits: i32 = Input::with_theme(&theme)
        .with_prompt("Enter Number of Commits to Perform")
        .validate_with(|input: &String| -> Result<(), &str> {
            input.parse::<i32>().map(|_| ()).map_err(|_| "Must be a number")
        })
        .interact_text()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
        .parse()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

    Ok(UserInput { repo, url, commits })
}
