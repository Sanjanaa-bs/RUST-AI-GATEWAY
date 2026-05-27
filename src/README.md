# 🚀 Rust AI API Gateway

> A high-performance AI inference gateway built with **Rust**, **Axum**, and **Ollama** for local LLM orchestration.

---

## ✨ Overview

This project is a Rust-based backend system that exposes REST APIs for AI inference.
The gateway receives prompts from users, forwards them to local language models through Ollama, and returns generated AI responses asynchronously.

---

## ⚡ Features

✅ Async backend using Tokio
✅ REST APIs with Axum
✅ JSON request & response handling
✅ Local LLM integration using Ollama
✅ AI prompt routing system
✅ Modular backend architecture
✅ Real-time AI response generation

---

## 🛠️ Tech Stack

| Technology | Purpose            |
| ---------- | ------------------ |
| Rust       | Backend Language   |
| Axum       | Web Framework      |
| Tokio      | Async Runtime      |
| Reqwest    | HTTP Client        |
| Serde      | JSON Serialization |
| Ollama     | Local LLM Serving  |
| TinyLlama  | AI Model           |

---

## 📡 API Endpoints

| Method | Endpoint    | Description          |
| ------ | ----------- | -------------------- |
| GET    | `/`         | Server Status        |
| GET    | `/health`   | Health Check         |
| POST   | `/generate` | Generate AI Response |

---

## 🧠 Example Request

```json
{
  "prompt": "Explain Rust programming"
}
```

---

## 🤖 Example Response

```json
{
  "response": "Rust is a systems programming language..."
}
```

---

## ▶️ Running the Project

### Start Ollama

```bash
ollama run tinyllama
```

### Run Backend

```bash
cargo run
```

Server runs on:

```text
http://localhost:3000
```

---

## 🏗️ Architecture

```text
User → Rust API Gateway → Ollama → TinyLlama → AI Response
```

---

## 🔥 Future Improvements

* JWT Authentication
* Redis Caching
* PostgreSQL Integration
* Docker Deployment
* WebSocket Streaming
* Multi-Model Routing

---

## 📌 Project Goal

The goal of this project is to explore:

* AI infrastructure engineering
* asynchronous backend systems
* local LLM orchestration
* high-performance API development using Rust

---

## 👨‍💻 Author

Built with Rust ❤️
