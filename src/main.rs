mod database;
mod embeddings;
mod indexer;
mod rag;
mod search;

use database::sqlite::{init_db, insert_chunk};
use embeddings::embedder::Embedder;
use indexer::chunker::chunk_file;
use indexer::scanner::scan_project;
use rag::llm::ask_llm;
use rag::prompt::build_prompt;
use rag::retriever::retrieve;

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

    // --- RAG ---
    let query = "How are code chunks saved to the database?";
    println!("Question: {}\n", query);

    let results = retrieve(&conn, &embedder, query, 3);

    println!("Retrieved {} chunks as context:", results.len());
    for r in &results {
        println!(
            "  - {} :: fn {} [score: {:.4}]",
            r.file_path, r.fn_name, r.score
        );
    }

    let prompt = build_prompt(query, &results);
    println!("\nAsking llama3.2...\n");

    let answer = ask_llm(&prompt);
    println!("Answer:\n{}", answer);
}
