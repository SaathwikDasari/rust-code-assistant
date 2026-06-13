use crate::database::sqlite::bytes_to_embedding;
use rusqlite::Connection;

pub struct SearchResult {
    pub file_path: String,
    pub fn_name: String,
    pub start_line: usize,
    pub end_line: usize,
    pub content: String,
    pub score: f32,
}

fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let mag_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let mag_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

    if mag_a == 0.0 || mag_b == 0.0 {
        return 0.0;
    }

    dot / (mag_a * mag_b)
}

pub fn search(conn: &Connection, query_embedding: &[f32], top_n: usize) -> Vec<SearchResult> {
    let mut stmt = conn
        .prepare("SELECT file_path, fn_name, start_line, end_line, content, embedding FROM chunks")
        .expect("failed to prepare query");

    let mut results: Vec<SearchResult> = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, i64>(2)?,
                row.get::<_, i64>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, Vec<u8>>(5)?,
            ))
        })
        .expect("query failed")
        .filter_map(|r| r.ok())
        .map(
            |(file_path, fn_name, start_line, end_line, content, embedding_bytes)| {
                let embedding = bytes_to_embedding(&embedding_bytes);
                let score = cosine_similarity(query_embedding, &embedding);
                SearchResult {
                    file_path,
                    fn_name,
                    start_line: start_line as usize,
                    end_line: end_line as usize,
                    content,
                    score,
                }
            },
        )
        .collect();

    results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    results.truncate(top_n);
    results
}
