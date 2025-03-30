#[macro_use]
mod log;

use git_url_parse::GitUrl;
use git2::Repository;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let current_dir = std::env::current_dir()?;
    let envrc_path = current_dir.join(".envrc");

    // check current directory's flake
    if current_dir.join("flake.nix").exists() {
        return write_use_flake(&envrc_path, ".");
    }

    let nix_activate_root = match env::var("NIX_ACTIVATE_ROOT") {
        Ok(var) if PathBuf::from(&var).is_dir() => {
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
        for remote in repo.remotes()?.iter().flatten() {
            let url = repo
                .find_remote(remote)
                .ok()
                .and_then(|remote| remote.url().map(String::from))
                .and_then(|url| GitUrl::parse(&url).ok());

            if let Some(url) = url {
                let host = url.host.unwrap_or("_".to_string());
                let path = PathBuf::from_iter([&nix_activate_root, &host, &url.fullname]);
                if path.join("flake.nix").exists() {
                    return write_use_flake(&envrc_path, path.to_str().unwrap());
                }
            }
        }
    }

    // check local-only flake
    let current_dir_str = current_dir.to_str().unwrap();
    let fullname = current_dir_str
        .split(std::path::MAIN_SEPARATOR_STR)
        .collect::<Vec<_>>()
        .iter()
        .rev()
        .take(3)
        .rev()
        .cloned()
        .collect::<Vec<_>>()
        .join(std::path::MAIN_SEPARATOR_STR);

    let path = PathBuf::from_iter([&nix_activate_root, &fullname]);
    if path.join("flake.nix").exists() {
        return write_use_flake(&envrc_path, path.to_str().unwrap());
    }

    warn!("no flake found");
    return Ok(());
}

fn write_use_flake(path: &Path, flake: &str) -> Result<(), Box<dyn std::error::Error>> {
    info!("use flake {}", flake);
    let mut file = File::create(path)?;
    match file.write_all(format!("use flake {}", flake).as_bytes()) {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("failed to write flake to {}", path.display());
            Err(Box::new(e))
        }
    }
}
