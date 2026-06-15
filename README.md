# rust-code-assistant

An AI-powered assistant that understands, searches, and explains Rust codebases using RAG (Retrieval-Augmented Generation), embeddings, and semantic search.

Ask questions about any Rust project in plain English and get accurate, context-aware answers.

```bash
$ cargo run -- ask "Where is the regex engine used?"

Question: Where is the regex engine used?

Retrieved 3 chunks as context:
  - crates/core/flags/hiargs.rs :: fn suggest_other_engine [score: 0.6943]
  - crates/globset/src/lib.rs   :: fn new_regex            [score: 0.6746]
  - tests/util.rs               :: fn setup_pcre2          [score: 0.6648]

Answer:
The regex engine is used in two places:
1. In `suggest_other_engine` to suggest an alternative engine based on the error.
2. In `new_regex` to compile a new regex pattern.
```

---

## How It Works

```
Rust Project
     │
     ▼
File Scanner          → finds all .rs files, skips target/ and .git/
     │
     ▼
Code Chunker          → splits files into functions using syn AST parsing
     │
     ▼
Embedding Generator   → turns each function into a 768-dim vector via Ollama
     │
     ▼
SQLite Storage        → persists chunks + embeddings in chunks.db
     │
     ▼
Similarity Search     → compares query embedding against stored vectors (cosine similarity)
     │
     ▼
RAG + LLM             → feeds top chunks as context to llama3.2, returns natural language answer
```

---

## Prerequisites

### 1. Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Ollama

```bash
curl -fsSL https://ollama.com/install.sh | sh
```

Pull the required models:

```bash
ollama pull nomic-embed-text   # embedding model
ollama pull llama3.2           # LLM for answering questions
```

Start the Ollama server:

```bash
ollama serve
```

---

## Installation

```bash
git clone https://github.com/SaathwikDasari/rust-code-assistant
cd rust-code-assistant
cargo build --release
```

---

## Usage

### Index a Rust project

Point the tool at any Rust project directory. This scans all `.rs` files, chunks them by function, generates embeddings, and stores everything in `chunks.db`.

```bash
cargo run -- index /path/to/rust/project
```

Example:

```bash
git clone https://github.com/BurntSushi/ripgrep
cargo run -- index ~/ripgrep
```

### Ask a question

Ask anything about the indexed codebase. No re-indexing needed.

```bash
cargo run -- ask "How does authentication work?"
cargo run -- ask "Where is the database connection created?"
cargo run -- ask "How does the search work?"
cargo run -- ask "Where is the regex engine used?"
```

---

## Project Structure

```
rust-code-assistant/
├── src/
│   ├── main.rs                  # CLI entry point
│   ├── cli/
│   │   └── commands.rs          # clap CLI definitions (index, ask)
│   ├── indexer/
│   │   ├── scanner.rs           # walks directories, reads .rs files
│   │   └── chunker.rs           # splits files into functions via syn
│   ├── embeddings/
│   │   └── embedder.rs          # calls Ollama nomic-embed-text
│   ├── database/
│   │   └── sqlite.rs            # stores and retrieves chunks + embeddings
│   ├── search/
│   │   └── similarity.rs        # cosine similarity search over embeddings
│   └── rag/
│       ├── retriever.rs         # embeds query, fetches top N chunks
│       ├── prompt.rs            # builds context-aware prompt
│       └── llm.rs               # calls Ollama llama3.2 for answers
```

---

## Tech Stack

| Layer | Technology |
|---|---|
| Language | Rust |
| Embeddings | Ollama + nomic-embed-text |
| LLM | Ollama + llama3.2 |
| Storage | SQLite (via rusqlite) |
| AST Parsing | syn |
| HTTP Client | reqwest |
| CLI | clap |

---

## What You Can Ask

- `"How does X work?"` — explains a feature or module
- `"Where is X defined?"` — finds relevant function definitions
- `"What files are related to X?"` — surfaces related code
- `"How are errors handled?"` — finds error handling patterns
- `"Where is the database connection created?"` — locates specific patterns

---

## Limitations

- Chunks at the function level only (structs, enums, traits not yet indexed)
- Re-indexing required if the project changes (`cargo run -- index .` again)
- Quality depends on the LLM and embedding model — larger models give better answers
- Works best on medium to large codebases where semantic signal is stronger

---

## Future Ideas

- Symbol navigation ("who calls this function?")
- Dependency graph generation
- AI code review
- Incremental re-indexing (only changed files)
- Interactive REPL mode
