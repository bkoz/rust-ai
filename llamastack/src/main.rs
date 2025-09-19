use clap::Parser;
use reqwest::Client;
use serde_json::{json, Value};

/// Small CLI for calling a local LlamaStack server.
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// The user prompt to send
    #[arg(short, long, default_value = "What is the capital city of Texas?")]
    prompt: String,

    /// Model id to use
    #[arg(short = 'm', long, default_value = "llama3.1:8b")]
    model_id: String,

    /// LlamaStack endpoint
    #[arg(short, long, default_value = "http://localhost:8321/v1/inference/chat-completion")]
    endpoint: String,
}

// Try to extract a textual response from common response shapes.
fn extract_text_from_value(v: &Value) -> Option<String> {
    // Top-level string
    if let Some(s) = v.as_str() {
        return Some(s.to_string());
    }

    // Common keys
    if let Some(field) = v.get("response").and_then(|x| x.as_str()) {
        return Some(field.to_string());
    }
    if let Some(field) = v.get("output").and_then(|x| x.as_str()) {
        return Some(field.to_string());
    }
    if let Some(field) = v.get("text").and_then(|x| x.as_str()) {
        return Some(field.to_string());
    }

    // choices -> [{ "message": { "content": "..." } }] (OpenAI-style)
    if let Some(choices) = v.get("choices").and_then(|c| c.as_array()) {
        if let Some(first) = choices.get(0) {
            if let Some(msg) = first.get("message") {
                if let Some(content) = msg.get("content").and_then(|x| x.as_str()) {
                    return Some(content.to_string());
                }
            }
            // choices[].delta.content (streaming pieces)
            if let Some(delta) = first.get("delta") {
                if let Some(content) = delta.get("content").and_then(|x| x.as_str()) {
                    return Some(content.to_string());
                }
            }
            // choices[].text
            if let Some(content) = first.get("text").and_then(|x| x.as_str()) {
                return Some(content.to_string());
            }
        }
    }

    // Some APIs return { "results": [ { "output": "..." } ] }
    if let Some(results) = v.get("results").and_then(|r| r.as_array()) {
        if let Some(first) = results.get(0) {
            if let Some(out) = first.get("output").and_then(|x| x.as_str()) {
                return Some(out.to_string());
            }
            if let Some(out) = first.get("text").and_then(|x| x.as_str()) {
                return Some(out.to_string());
            }
            // Some LlamaStack-like servers put text inside `content` array
            if let Some(contents) = first.get("content").and_then(|c| c.as_array()) {
                let mut combined = String::new();
                for item in contents {
                    if let Some(s) = item.as_str() {
                        combined.push_str(s);
                    }
                }
                if !combined.is_empty() {
                    return Some(combined);
                }
            }
        }
    }

    None
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let client = Client::new();

    // Build request body using CLI args
    let request_body = json!({
        "model_id": args.model_id,
        "messages": [
            { "role": "user", "content": args.prompt },
        ],
        "max_tokens": 512
    });

    let url = args.endpoint;

    let resp = client.post(&url).json(&request_body).send().await?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body_text = resp.text().await.unwrap_or_else(|_| "<unable to read body>".to_string());
        eprintln!("HTTP error {}: {}", status, body_text);
        std::process::exit(1);
    }

    let value = resp.json::<Value>().await?;

    // LlamaStack returns { "completion_message": { "content": "..." } }
    if let Some(cmsg) = value.get("completion_message") {
        if let Some(content) = cmsg.get("content").and_then(|c| c.as_str()) {
            println!("Response: {}", content);
            return Ok(());
        }
    }

    if let Some(text) = extract_text_from_value(&value) {
        println!("Response: {}", text);
    } else {
        println!("Full response JSON: {}", value);
    }
    Ok(())
}