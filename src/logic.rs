use std::io;
use std::fs;
use std::path::Path;
use std::process::Command;

pub fn update() -> io::Result<()> {
    if Path::new("a.txt").exists() {
        fs::rename("a.txt", "b.txt")?;
    } else if Path::new("b.txt").exists() {
        fs::rename("b.txt", "a.txt")?;
    } else {
        for entry in fs::read_dir(".")? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.file_name().unwrap() != ".git" {
                fs::remove_file(&path)?;
            }
        }
        fs::File::create("a.txt")?;
    }
    Ok(())
}

pub fn pull(repo: &str, url: &str) -> io::Result<()> {
    if !Path::new(repo).exists() {
        println!("Cloning {} into {}...", url, repo);
        let status = Command::new("git")
            .arg("clone")
            .arg(url)
            .arg(repo)
            .status()?;
        if !status.success() {
            return Err(io::Error::new(io::ErrorKind::Other, "git clone failed"));
        }
    }

    std::env::set_current_dir(repo)?;
    
    if !Path::new(".git").exists() {
        Command::new("git").arg("init").status()?;
        if !url.is_empty() {
            Command::new("git").args(["remote", "add", "origin", url]).status()?;
            Command::new("git").args(["branch", "-M", "main"]).status()?;
        }
    }
    Ok(())
}

pub fn push(count: &mut i32) -> io::Result<()> {
    Command::new("git").arg("add").arg(".").status()?;
    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(format!("commit {}", count))
        .status()?;
    Command::new("git").args(["push", "origin", "main"]).status()?;
    *count += 1;
    Ok(())
}
