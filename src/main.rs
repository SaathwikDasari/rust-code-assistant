use std::fs;
use walkdir::WalkDir;

#[derive(Debug)]
struct CodeFile {
    path: String,
    contents: String,
}

fn main() {
    let mut files: Vec<CodeFile> = Vec::new();

    for entry in WalkDir::new(".").into_iter().filter_entry(|e| {
        let name = e.file_name().to_string_lossy();
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
                    contents: content,
                })
            }
        }
    }

    for file in &files {
        println!("Found: {} ({} bytes)", file.path, file.contents.len());
    }
}
