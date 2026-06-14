use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct GenerateRequest<'a> {
    model: &'a str,
    prompt: &'a str,
    stream: bool,
}

#[derive(Deserialize)]
struct GenerateResponse {
    response: String,
}

pub fn ask_llm(prompt: &str) -> String {
    let client = reqwest::blocking::Client::new();

    let req = GenerateRequest {
        model: "llama3.2",
        prompt,
        stream: false,
    };

    let resp: GenerateResponse = client
        .post("http://localhost:11434/api/generate")
        .json(&req)
        .send()
        .expect("request to Ollama failed")
        .json()
        .expect("Failed to parse response");

    resp.response
}
