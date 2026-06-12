use std::fs;
use walkdir::WalkDir;

#[derive(Debug)]
pub struct CodeFile {
    pub path: String,
    pub content: String,
}

pub fn scan_project(root: &str) -> Vec<CodeFile> {
    let mut files = Vec::new();
    for entry in WalkDir::new(root).into_iter().filter_entry(|e| {
        let name = e.file_name().to_str().unwrap_or("");
        name != "target" && name != ".git"
    }) {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => continue,
        };

        if entry.path().extension().and_then(|e| e.to_str()) == Some("rs") {
            if let Ok(content) = fs::read_to_string(entry.path()) {
                files.push(CodeFile {
                    path: entry.path().display().to_string(),
                    content,
                });
            }
        }
    }
    files
}
