use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct EmbedRequest<'a> {
    model: &'a str,
    prompt: &'a str,
}

#[derive(Deserialize)]
struct EmbedResponse {
    embedding: Vec<f32>,
}

pub struct Embedder {
    client: reqwest::blocking::Client,
}

impl Embedder {
    pub fn new() -> Self {
        Embedder {
            client: reqwest::blocking::Client::new(),
        }
    }

    pub fn embed(&self, text: &str) -> Vec<f32> {
        let req = EmbedRequest {
            model: "nomic-embed-text",
            prompt: text,
        };

        let resp: EmbedResponse = self
            .client
            .post("http://localhost:11434/api/embeddings")
            .json(&req)
            .send()
            .expect("request to Ollama failed")
            .json()
            .expect("failed to parse response");

        resp.embedding
    }
}
