use anyhow::{Context, Result};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

pub fn resolve(cli_arg: Option<&Path>) -> Result<PathBuf> {
    if let Some(p) = cli_arg {
        return Ok(expand(p));
    }
    if let Ok(v) = env::var("YTMP3_DIR") {
        if !v.is_empty() {
            return Ok(expand(Path::new(&v)));
        }
    }
    if let Some(p) = wsl_windows_downloads() {
        return Ok(p);
    }
    dirs::download_dir()
        .or_else(|| dirs::home_dir().map(|h| h.join("Downloads")))
        .context("cannot determine a Downloads directory")
}

fn expand(p: &Path) -> PathBuf {
    let s = p.to_string_lossy();
    if let Some(rest) = s.strip_prefix("~/") {
        if let Some(home) = dirs::home_dir() {
            return home.join(rest);
        }
    }
    if s == "~" {
        if let Some(home) = dirs::home_dir() {
            return home;
        }
    }
    p.to_path_buf()
}

fn is_wsl() -> bool {
    fs::read_to_string("/proc/version")
        .map(|s| s.to_ascii_lowercase().contains("microsoft"))
        .unwrap_or(false)
}

fn wsl_windows_downloads() -> Option<PathBuf> {
    if !is_wsl() {
        return None;
    }
    let users = Path::new("/mnt/c/Users");
    if !users.exists() {
        return None;
    }

    for hint in ["WSL_USER", "USER", "USERNAME", "LOGNAME"] {
        if let Ok(name) = env::var(hint) {
            if name.is_empty() {
                continue;
            }
            let p = users.join(&name).join("Downloads");
            if p.is_dir() {
                return Some(p);
            }
        }
    }

    let skip = [
        "Public",
        "Default",
        "Default User",
        "All Users",
        "desktop.ini",
    ];
    fs::read_dir(users)
        .ok()?
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().map(|t| t.is_dir()).unwrap_or(false))
        .filter(|e| !skip.iter().any(|s| e.file_name() == *s))
        .map(|e| e.path().join("Downloads"))
        .find(|p| p.is_dir())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    // env::set_var is process-global; serialize tests that touch env.
    static ENV_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn cli_arg_wins() {
        let _g = ENV_LOCK.lock().unwrap();
        let tmp = std::env::temp_dir().join("ytmp3_test_cli");
        let out = resolve(Some(&tmp)).unwrap();
        assert_eq!(out, tmp);
    }

    #[test]
    fn env_var_used_when_no_cli() {
        let _g = ENV_LOCK.lock().unwrap();
        let tmp = std::env::temp_dir().join("ytmp3_test_env");
        env::set_var("YTMP3_DIR", &tmp);
        let out = resolve(None).unwrap();
        env::remove_var("YTMP3_DIR");
        assert_eq!(out, tmp);
    }

    #[test]
    fn tilde_expansion() {
        let home = dirs::home_dir().unwrap();
        let out = expand(Path::new("~/Music"));
        assert_eq!(out, home.join("Music"));
    }
}
