use std::fs;
use std::path::PathBuf;

pub fn get_git_info(cwd: Option<&String>) -> (Option<String>, Option<String>) {
    let Some(cwd_str) = cwd else {
        return (None, None);
    };
    let mut current_dir = PathBuf::from(cwd_str);

    loop {
        let git_dir = current_dir.join(".git");
        if git_dir.exists() && git_dir.is_dir() {
            let repo_name = current_dir
                .file_name()
                .map(|n| n.to_string_lossy().into_owned());

            let head_path = git_dir.join("HEAD");
            let branch = if let Ok(contents) = fs::read_to_string(head_path) {
                parse_git_head(&contents)
            } else {
                None
            };

            return (repo_name, branch);
        }

        if !current_dir.pop() {
            break;
        }
    }

    (None, None)
}

fn parse_git_head(contents: &str) -> Option<String> {
    let contents = contents.trim();
    if let Some(stripped) = contents.strip_prefix("ref: ") {
        stripped.split('/').next_back().map(|s| s.to_string())
    } else {
        Some(contents.chars().take(7).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_git_head() {
        assert_eq!(parse_git_head("ref: refs/heads/main"), Some("main".to_string()));
        assert_eq!(parse_git_head("ref: refs/heads/feature/branch"), Some("branch".to_string()));
        assert_eq!(parse_git_head("a1b2c3d4e5f6"), Some("a1b2c3d".to_string()));
    }
}
