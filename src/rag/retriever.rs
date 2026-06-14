use crate::embeddings::embedder::Embedder;
use crate::search::similarity::{SearchResult, search};
use rusqlite::Connection;

pub fn retrieve(
    conn: &Connection,
    embedder: &Embedder,
    query: &str,
    top_n: usize,
) -> Vec<SearchResult> {
    let query_embeddings = embedder.embed(query);
    search(conn, &query_embeddings, top_n)
}
