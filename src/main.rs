mod embeddings;
mod indexer;

use embeddings::embedder::Embedder;
use indexer::chunker::chunk_file;
use indexer::scanner::scan_project;

fn main() {
    let files = scan_project(".");
    let mut all_chunks = Vec::new();

    for file in &files {
        let chunks = chunk_file(&file.path, &file.content);
        all_chunks.extend(chunks);
    }

    println!(
        "Found {} function chunks. Generating embeddings...",
        all_chunks.len()
    );

    let embedder = Embedder::new();

    for chunk in &all_chunks {
        let vector = embedder.embed(&chunk.content);
        println!(
            "{} :: fn {} -> embedding dim = {}, first values = {:?}",
            chunk.file_path,
            chunk.fn_name,
            vector.len(),
            &vector[..5.min(vector.len())]
        );
    }
}
