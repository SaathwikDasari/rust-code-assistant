use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rust-code-assistant")]
#[command(about = "AI-powered Rust codebase assistant")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Index a Rust project directory
    Index {
        /// Path to the Rust project to index
        path: String,
    },
    /// Ask a question about the indexed codebase
    Ask {
        /// Your question
        question: String,
    },
}
