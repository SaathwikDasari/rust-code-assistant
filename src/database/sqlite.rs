use crate::indexer::chunker::CodeChunk;
use rusqlite::{Connection, params};

pub fn init_db(path: &str) -> Connection {
    let conn = Connection::open(path).expect("failed to open database");

    conn.execute("DROP TABLE IF EXISTS chunks", [])
        .expect("failed to drop existing table");

    conn.execute(
        "CREATE TABLE chunks (
            id INTEGER PRIMARY KEY,
            file_path TEXT,
            fn_name TEXT,
            start_line INTEGER,
            end_line INTEGER,
            content TEXT,
            embedding BLOB
        )",
        [],
    )
    .expect("failed to create table");

    conn
}

pub fn insert_chunk(conn: &Connection, chunk: &CodeChunk, embedding: &[f32]) {
    let embedding_bytes: Vec<u8> = embedding.iter().flat_map(|f| f.to_le_bytes()).collect();

    conn.execute(
        "INSERT INTO chunks (file_path, fn_name, start_line, end_line, content, embedding)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            chunk.file_path,
            chunk.fn_name,
            chunk.start_line as i64,
            chunk.end_line as i64,
            chunk.content,
            embedding_bytes,
        ],
    )
    .expect("failed to insert chunk");
}

pub fn bytes_to_embedding(bytes: &[u8]) -> Vec<f32> {
    bytes
        .chunks_exact(4)
        .map(|b| f32::from_le_bytes([b[0], b[1], b[2], b[3]]))
        .collect()
}
