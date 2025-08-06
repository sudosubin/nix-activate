#[macro_use]
mod log;

use git_url_parse::GitUrl;
use git2::Repository;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let current_dir = env::current_dir()?;
    let envrc_path = current_dir.join(".envrc");

    // check current directory's flake
    if current_dir.join("flake.nix").exists() {
        return write_use_flake(&envrc_path, ".");
    }

    let nix_activate_root = match env::var("NIX_ACTIVATE_ROOT") {
        Ok(var) if Path::new(&var).is_dir() => {
            info!("try external flakes from {}", var);
            var
        }
        Ok(_) => {
            error!("set NIX_ACTIVATE_ROOT environment variable to a valid directory path");
            return Ok(());
        }
        Err(_) => {
            error!("set NIX_ACTIVATE_ROOT environment variable to use external flake");
            return Ok(());
        }
    };

    // check git directory's flake
    if let Ok(repo) = Repository::open(&current_dir) {
        // Get the relative path from repository root to current directory
        let repo_root = repo.workdir().unwrap();
        let relative_path = current_dir
            .strip_prefix(repo_root)
            .ok()
            .and_then(|p| p.to_str())
            .unwrap_or("");

        for remote in repo.remotes()?.iter().flatten() {
            let url = repo
                .find_remote(remote)
                .ok()
                .and_then(|remote| remote.url().map(String::from))
                .and_then(|url| GitUrl::parse(&url).ok());

            if let Some(url) = url {
                let host = url.host.as_deref().unwrap_or("_");
                let path =
                    PathBuf::from_iter([&nix_activate_root, host, &url.fullname, relative_path]);

                if path.join("flake.nix").exists() {
                    if let Some(path_str) = path.to_str() {
                        return write_use_flake(&envrc_path, path_str);
                    }
                }
            }
        }
    }

    // check local-only flake
    let components: Vec<_> = current_dir.components().rev().take(3).collect();
    let fullname = components.into_iter().rev().collect::<PathBuf>();
    let path = PathBuf::from_iter([Path::new(&nix_activate_root), &fullname]);
    if path.join("flake.nix").exists() {
        return write_use_flake(&envrc_path, path.to_str().unwrap());
    }

    warn!("no flake found");
    Ok(())
}

fn write_use_flake(path: &Path, flake: &str) -> Result<(), Box<dyn std::error::Error>> {
    info!("use flake {}", flake);
    let mut file = File::create(path)?;
    match write!(file, "use flake {}", flake) {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("failed to write flake to {}", path.display());
            Err(Box::new(e))
        }
    }
}
