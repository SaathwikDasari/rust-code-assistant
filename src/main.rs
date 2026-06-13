mod database;
mod embeddings;
mod indexer;
mod search;

use database::sqlite::{init_db, insert_chunk};
use embeddings::embedder::Embedder;
use indexer::chunker::chunk_file;
use indexer::scanner::scan_project;
use search::similarity::search;

fn main() {
    // --- Indexing ---
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
    let conn = init_db("chunks.db");

    for chunk in &all_chunks {
        let vector = embedder.embed(&chunk.content);
        insert_chunk(&conn, chunk, &vector);
        println!("Stored: {} :: fn {}", chunk.file_path, chunk.fn_name);
    }

    println!("Done. {} chunks stored in chunks.db\n", all_chunks.len());

    // --- Search ---
    let query = "How are code chunks saved to the database?";
    println!("Query: {}\n", query);

    let query_embedding = embedder.embed(query);
    let results = search(&conn, &query_embedding, 3);

    println!("Top results:");
    for (i, result) in results.iter().enumerate() {
        println!(
            "{}. {} :: fn {} (lines {}-{}) [score: {:.4}]",
            i + 1,
            result.file_path,
            result.fn_name,
            result.start_line,
            result.end_line,
            result.score
        );
    }
}
