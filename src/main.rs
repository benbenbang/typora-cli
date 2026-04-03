use std::env;
use std::path::Path;
use std::process::{self, Command};

/// Returns `None` if the argument means "open current directory" (no arg, empty, or ".").
/// Otherwise returns the path the user provided, unchanged.
pub fn parse_target(document: &str) -> Option<&str> {
    if document.is_empty() || document == "." {
        None
    } else {
        Some(document)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let dry_run = args.iter().any(|a| a == "--dry-run");

    let document = args
        .iter()
        .skip(1)
        .find(|a| *a != "--dry-run")
        .map(String::as_str)
        .unwrap_or("");

    match parse_target(document) {
        None => {
            // No argument or "." — open the current directory.
            if !dry_run {
                open_typora(".");
            }
        }
        Some(target) => {
            let path = Path::new(target);

            if !path.exists() {
                if let Some(parent) = path.parent()
                    && let Err(e) = std::fs::create_dir_all(parent)
                {
                    eprintln!("Error creating directories '{}': {}", parent.display(), e);
                    process::exit(1);
                }
                if let Err(e) = std::fs::File::create(path) {
                    eprintln!("Error creating file '{}': {}", path.display(), e);
                    process::exit(1);
                }
            }

            if !dry_run {
                // Canonicalize so Typora always receives an absolute path,
                // which is more reliable across platforms.
                let abs = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
                open_typora(&abs.to_string_lossy());
            }
        }
    }
}

fn typora_installed() -> bool {
    if cfg!(target_os = "macos") {
        let home = std::env::var("HOME").unwrap_or_default();
        Path::new("/Applications/Typora.app").exists()
            || Path::new(&format!("{}/Applications/Typora.app", home)).exists()
    } else {
        let exe = if cfg!(target_os = "windows") {
            "typora.exe"
        } else {
            "typora"
        };
        std::env::var_os("PATH")
            .map(|paths| std::env::split_paths(&paths).any(|dir| dir.join(exe).is_file()))
            .unwrap_or(false)
    }
}

fn open_typora(target: &str) {
    if !typora_installed() {
        eprintln!("Error: Typora does not appear to be installed.");
        if cfg!(target_os = "macos") {
            eprintln!("Expected: /Applications/Typora.app or ~/Applications/Typora.app");
        } else {
            eprintln!("Expected: `typora` binary on PATH");
        }
        process::exit(1);
    }

    let result = if cfg!(target_os = "macos") {
        Command::new("open").args(["-a", "typora", target]).status()
    } else {
        // Linux and Windows: Typora is expected to be on PATH.
        // On Windows, `typora` resolves to typora.exe when installed normally.
        Command::new("typora").arg(target).status()
    };

    match result {
        Ok(s) if s.success() => {}
        Ok(s) => process::exit(s.code().unwrap_or(1)),
        Err(e) => {
            eprintln!("Failed to launch Typora: {}", e);
            process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::parse_target;

    #[test]
    fn empty_string_is_sentinel() {
        assert_eq!(parse_target(""), None);
    }

    #[test]
    fn dot_is_sentinel() {
        assert_eq!(parse_target("."), None);
    }

    #[test]
    fn bare_name_passes_through() {
        assert_eq!(parse_target("notes"), Some("notes"));
    }

    #[test]
    fn name_with_extension_passes_through() {
        assert_eq!(parse_target("notes.md"), Some("notes.md"));
    }

    #[test]
    fn relative_path_passes_through() {
        assert_eq!(parse_target("./docs/guide.md"), Some("./docs/guide.md"));
    }

    #[test]
    fn parent_dir_path_passes_through() {
        assert_eq!(parse_target("../notes"), Some("../notes"));
    }

    #[test]
    fn absolute_path_passes_through() {
        assert_eq!(parse_target("/tmp/foo.md"), Some("/tmp/foo.md"));
    }

    #[test]
    fn name_with_spaces_passes_through() {
        assert_eq!(parse_target("my notes"), Some("my notes"));
    }

    #[test]
    fn name_without_md_passes_through_unchanged() {
        // No longer auto-appends .md — user gets exactly what they asked for.
        assert_eq!(parse_target("readme.txt"), Some("readme.txt"));
    }
}
