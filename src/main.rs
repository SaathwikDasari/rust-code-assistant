mod cli;
mod database;
mod embeddings;
mod indexer;
mod rag;
mod search;

use clap::Parser;
use cli::commands::{Cli, Commands};
use database::sqlite::{init_db, insert_chunk};
use embeddings::embedder::Embedder;
use indexer::chunker::chunk_file;
use indexer::scanner::scan_project;
use rag::llm::ask_llm;
use rag::prompt::build_prompt;
use rag::retriever::retrieve;
use rusqlite::Connection;

const DB_PATH: &str = "chunks.db";

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Index { path } => {
            println!("Indexing project at: {}", path);

            let files = scan_project(&path);
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
            let conn = init_db(DB_PATH);

            for chunk in &all_chunks {
                let vector = embedder.embed(&chunk.content);
                insert_chunk(&conn, chunk, &vector);
                println!("Stored: {} :: fn {}", chunk.file_path, chunk.fn_name);
            }

            println!("Done. {} chunks stored in {}", all_chunks.len(), DB_PATH);
        }

        Commands::Ask { question } => {
            println!("Question: {}\n", question);

            let conn =
                Connection::open(DB_PATH).expect("Failed to open chunks.db — run `index` first");

            let embedder = Embedder::new();
            let results = retrieve(&conn, &embedder, &question, 3);

            if results.is_empty() {
                println!("No results found. Have you indexed a project yet?");
                return;
            }

            println!("Retrieved {} chunks as context:", results.len());
            for r in &results {
                println!(
                    "  - {} :: fn {} [score: {:.4}]",
                    r.file_path, r.fn_name, r.score
                );
            }

            let prompt = build_prompt(&question, &results);
            println!("\nAsking llama3.2...\n");

            let answer = ask_llm(&prompt);
            println!("Answer:\n{}", answer);
        }
    }
}
