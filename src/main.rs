mod indexer;

use indexer::chunker::chunk_file;
use indexer::scanner::scan_project;

fn main() {
    let files = scan_project(".");
    let mut all_chunks = Vec::new();

    for file in &files {
        let chunks = chunk_file(&file.path, &file.content);
        all_chunks.extend(chunks);
    }

    for chunk in &all_chunks {
        println!(
            "{} :: fn {} (lines {}-{})",
            chunk.file_path, chunk.fn_name, chunk.start_line, chunk.end_line
        );

        println!("---\n{}\n---\n", chunk.content);
    }
}
