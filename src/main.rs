use tower_http::cors::CorsLayer;
use axum::{
    routing::{get, post},
    Json,
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Deserialize)]
struct PromptRequest {
    prompt: String,
}

#[derive(Serialize)]
struct PromptResponse {
    response: String,
}

async fn home() -> &'static str {
    "AI Gateway Running 🚀"
}

async fn health() -> Json<serde_json::Value> {
    Json(json!({
        "status": "healthy"
    }))
}

async fn generate(
    Json(payload): Json<PromptRequest>,
) -> Json<PromptResponse> {

    let client = reqwest::Client::new();

    let ollama_request = json!({
        "model": "tinyllama",
        "prompt": payload.prompt,
        "stream": false
    });

    let response = client
        .post("http://localhost:11434/api/generate")
        .json(&ollama_request)
        .send()
        .await;

    match response {
        Ok(res) => {
            let json_response: Value =
                res.json().await.unwrap();

            let ai_response = json_response["response"]
                .as_str()
                .unwrap_or("No response");

            Json(PromptResponse {
                response: ai_response.to_string(),
            })
        }

        Err(_) => Json(PromptResponse {
            response: "Failed to connect to Ollama".to_string(),
        }),
    }
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::permissive();
    let app = Router::new()
        .route("/", get(home))
        .route("/health", get(health))
        .route("/generate", post(generate))
         .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("Server running on http://localhost:3000");

    axum::serve(listener, app)
        .await
        .unwrap();
}