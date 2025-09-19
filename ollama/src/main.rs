use reqwest::Client;
use serde::{Deserialize};
use serde_json::json;

// Minimal response type matching the API's JSON structure.
#[derive(Debug, Deserialize)]
struct GenerateResponse {
    // Adjust the field name to match the actual API response. This assumes the API
    // returns { "response": "..." } — change as needed.
    response: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let request_body = json!({
        "model": "llama3.1:8b",
        "prompt": "What is the capital city of Texas?",
        "stream": false
    });

    let res = client.post("http://localhost:11434/api/generate")
        .json(&request_body)
        .send()
        .await?
        .json::<GenerateResponse>()
        .await?;

    println!("Response: {}", res.response);
    Ok(())
}