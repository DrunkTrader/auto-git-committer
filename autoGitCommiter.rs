use std::fs;
use std::io;
use std::process::Command;
use std::path::Path;

fn update() -> io::Result<()> {
    if Path::new("a.txt").exists() {
        fs::rename("a.txt", "b.txt").map_err(|e| {
            eprintln!("Failed to rename a.txt to b.txt: {}", e);
            e
        })?;
    } else if Path::new("b.txt").exists() {
        fs::rename("b.txt", "a.txt").map_err(|e| {
            eprintln!("Failed to rename b.txt to a.txt: {}", e);
            e
        })?;
    } else {
        for entry in fs::read_dir(".")? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.file_name().unwrap() != ".git" {
                fs::remove_file(&path).map_err(|e| {
                    eprintln!("Failed to delete file {:?}: {}", path, e);
                    e
                })?;
            }
        }
        fs::File::create("a.txt").map_err(|e| {
            eprintln!("Failed to create a.txt: {}", e);
            e
        })?;
    }
    Ok(())
}

fn pull(repo: &str, url: &str) -> io::Result<()> {
    // Case 1: Repository needs to be cloned
    if !Path::new(repo).exists() {
        println!("Repository doesn't exist. Cloning from {}...", url);
        let status = Command::new("git")
            .arg("clone")
            .arg(url)
            .status()
            .map_err(|e| {
                eprintln!("Failed to execute git clone: {}", e);
                e
            })?;
        
        if !status.success() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "git clone failed - check URL and permissions",
            ));
        }
        println!("Repository cloned successfully.");
    }
    
    // Change to repository directory
    std::env::set_current_dir(repo).map_err(|e| {
        eprintln!("Failed to change directory to {}: {}", repo, e);
        e
    })?;

    // Case 2: Empty directory needs Git initialization
    if !Path::new(".git").exists() {
        println!("Initializing new Git repository in {}...", repo);
        
        // Initialize git repository
        let init_status = Command::new("git")
            .arg("init")
            .status()
            .map_err(|e| {
                eprintln!("Failed to initialize Git repository: {}", e);
                e
            })?;
        
        if !init_status.success() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "git init failed",
            ));
        }
        
        // Create basic repository files
        fs::write("README.md", format!("# {}\n\nAutomatic repository created for commit automation.", repo))
            .map_err(|e| {
                eprintln!("Failed to create README.md: {}", e);
                e
            })?;
        
        // Create .gitignore with common patterns
        fs::write(".gitignore", "# Generated .gitignore file\n*.log\ntmp/\n.DS_Store\n").map_err(|e| {
            eprintln!("Failed to create .gitignore: {}", e);
            e
        })?;
        
        // Make initial commit
        let add_status = Command::new("git")
            .args(["add", "README.md", ".gitignore"])
            .status()
            .map_err(|e| {
                eprintln!("Failed to add initial files: {}", e);
                e
            })?;
            
        if !add_status.success() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "git add failed for initial files",
            ));
        }
        
        let commit_status = Command::new("git")
            .args(["commit", "-m", "Initial repository setup"])
            .status()
            .map_err(|e| {
                eprintln!("Failed to create initial commit: {}", e);
                e
            })?;
            
        if !commit_status.success() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Initial git commit failed",
            ));
        }
        
        // Configure remote if URL was provided
        if !url.is_empty() {
            println!("Setting up remote origin at {}...", url);
            let remote_status = Command::new("git")
                .args(["remote", "add", "origin", url])
                .status()
                .map_err(|e| {
                    eprintln!("Failed to add remote: {}", e);
                    e
                })?;
                
            if !remote_status.success() {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Failed to add git remote",
                ));
            }
            
            // Configure branch to track remote
            let branch_status = Command::new("git")
                .args(["branch", "-M", "main"])
                .status()
                .map_err(|e| {
                    eprintln!("Failed to create main branch: {}", e);
                    e
                })?;
                
            if !branch_status.success() {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Failed to configure main branch",
                ));
            }
        }
        
        println!("Repository initialized successfully.");
    }
    // Case 3: Existing repository - check remote
    else {
        // Check if remote needs to be added
        let remote_check = Command::new("git")
            .args(["remote", "get-url", "origin"])
            .output()
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to check remote"))?;
            
        if !remote_check.status.success() && !url.is_empty() {
            println!("Adding remote origin at {}...", url);
            let remote_add = Command::new("git")
                .args(["remote", "add", "origin", url])
                .status()?;
                
            if !remote_add.success() {
                eprintln!("Warning: Failed to add remote origin");
            }
        }
    }

    Ok(())
}

fn push(count: &mut i32) -> io::Result<()> {
    let status = Command::new("git")
        .arg("add")
        .arg(".")
        .status()
        .map_err(|e| {
            eprintln!("Failed to execute git add: {}", e);
            e
        })?;
    if !status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "git add failed",
        ));
    }

    let status = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(format!("commit {}", count))
        .status()
        .map_err(|e| {
            eprintln!("Failed to execute git commit: {}", e);
            e
        })?;
    if !status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "git commit failed",
        ));
    }

    let status = Command::new("git")
        .arg("push")
        .arg("origin")
        .arg("main") // Use "main" instead of "master"
        .status()
        .map_err(|e| {
            eprintln!("Failed to execute git push: {}", e);
            e
        })?;
    if !status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "git push failed",
        ));
    }

    *count += 1;
    Ok(())
}

fn main() -> io::Result<()> {
    let mut repo = String::new();
    let mut url = String::new();
    let mut comm = String::new();

    println!("Enter Git Repository name: ");
    io::stdin().read_line(&mut repo)?;
    let repo = repo.trim();

    println!("Enter Git Repository URL: ");
    io::stdin().read_line(&mut url)?;
    let url = url.trim();

    println!("Enter Number Of Commits To Perform: ");
    io::stdin().read_line(&mut comm)?;
    let comm: i32 = comm.trim().parse().expect("Please enter a valid number");

    pull(repo, url)?;
    let mut count = 1;

    for _ in 0..comm {
        update()?;
        push(&mut count)?;
    }

    Ok(())
}