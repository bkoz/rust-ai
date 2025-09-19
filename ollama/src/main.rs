use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use clap::Parser;


/// Simple program to query a model API
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// API URL (e.g. http://localhost:11434/api/generate)
    #[arg(short, long, default_value = "http://localhost:11434/api/generate")]
    url: String,

    /// Model name (e.g. llama3.1:8b)
    #[arg(short, long, default_value = "llama3.1:8b")]
    model: String,
}

#[derive(Debug, Deserialize)]
struct GenerateResponse {
    response: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let client = Client::new();
    let request_body = json!({
        "model": args.model,
        "prompt": "What is the capital city of Texas?",
        "stream": false
    });

    let res = client.post(&args.url)
        .json(&request_body)
        .send()
        .await?
        .json::<GenerateResponse>()
        .await?;

    println!("Response: {}", res.response);
    Ok(())
}