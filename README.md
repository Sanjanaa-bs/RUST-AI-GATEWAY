# 🚀 Rust AI Gateway

> A full-stack AI inference platform built with Rust, Axum, Next.js, and Ollama for local LLM orchestration.

---

# ✨ Overview

Rust AI Gateway is a high-performance AI infrastructure project that exposes REST APIs for AI inference and connects them with a modern frontend interface.

The system receives prompts from users, routes them through a Rust backend, forwards requests to local LLMs using Ollama, and returns generated AI responses in real time.

---

# 🏗️ Architecture

```text
Frontend (Next.js)
        ↓
Rust API Gateway
        ↓
Ollama
        ↓
TinyLlama AI Model
```

---

# ⚡ Features

✅ Full-stack AI application
✅ Rust async backend using Tokio
✅ REST APIs with Axum
✅ Next.js frontend interface
✅ JSON request & response handling
✅ Local LLM integration using Ollama
✅ Real-time AI response generation
✅ Modular backend architecture
✅ AI prompt routing system

---

# 🛠️ Tech Stack

| Technology   | Purpose            |
| ------------ | ------------------ |
| Rust         | Backend Language   |
| Axum         | Web Framework      |
| Tokio        | Async Runtime      |
| Reqwest      | HTTP Client        |
| Serde        | JSON Serialization |
| Next.js      | Frontend Framework |
| TypeScript   | Frontend Language  |
| Tailwind CSS | UI Styling         |
| Ollama       | Local LLM Serving  |
| TinyLlama    | AI Model           |

---

# 📡 API Endpoints

| Method | Endpoint    | Description          |
| ------ | ----------- | -------------------- |
| GET    | `/`         | Server Status        |
| GET    | `/health`   | Health Check         |
| POST   | `/generate` | Generate AI Response |

---

# 🧠 Example Request

```json
{
  "prompt": "Explain Rust programming"
}
```

---

# 🤖 Example Response

```json
{
  "response": "Rust is a systems programming language..."
}
```

---

# ▶️ Running the Project

## 1. Start Ollama

```bash
ollama run tinyllama
```

---

## 2. Start Rust Backend

```bash
cargo run
```

Backend runs on:

```text
http://localhost:3000
```

---

## 3. Start Frontend

```bash
cd frontend
npm run dev -- -p 3002
```

Frontend runs on:

```text
http://localhost:3002
```

---

# 📂 Project Structure

```text
rust-ai-gateway/
│
├── src/              # Rust backend
├── frontend/         # Next.js frontend
├── Cargo.toml
├── README.md
└── .gitignore
```

---

# 🔥 Future Improvements

* JWT Authentication
* Redis Caching
* PostgreSQL Integration
* Docker Deployment
* WebSocket Streaming
* Multi-Model Routing
* AI Agent Support

---

# 📌 Project Goal

The goal of this project is to explore:

* AI infrastructure engineering
* asynchronous backend systems
* local LLM orchestration
* full-stack AI application development
* high-performance APIs using Rust

---

# 👨‍💻 Author

Built with Rust ❤️
