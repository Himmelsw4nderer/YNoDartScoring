# YNoDartScoring

A modern dart scoring application with a Rust backend and Yew frontend.

## 🎯 Project Structure

This is a monorepo containing both frontend and backend:

```
YNoDartScoring/
├── frontend/          # Yew (Rust WASM) frontend
├── backend/           # Rocket (Rust) backend API
├── Dockerfile         # Multi-stage Docker build
└── docker-compose.yml # Docker Compose configuration
```

## 🚀 Development

### Prerequisites

- Rust 1.92+
- Cargo
- Trunk (for frontend): `cargo install trunk`
- wasm32 target: `rustup target add wasm32-unknown-unknown`
- Docker & Docker Compose (optional)

### Frontend Development

```bash
cd frontend
trunk serve
```

The frontend will be available at `http://localhost:8080`

### Backend Development

```bash
cd backend
cargo run
```

The backend API will be available at `http://localhost:8000`

### Building

#### Frontend
```bash
cd frontend
trunk build --release
```

#### Backend
```bash
cd backend
cargo build --release
```

## 🐳 Docker

Build and run with Docker Compose:

```bash
docker-compose up --build
```

Or build the Docker image manually:

```bash
docker build -t ynodartscore .
docker run -p 8000:8000 ynodartscore
```

## 📦 Tech Stack

### Frontend
- **Yew** - Rust framework for building web apps with WebAssembly
- **Tailwind CSS** - Utility-first CSS framework
- **web-sys** - Rust bindings for Web APIs
- **gloo** - Modular toolkit for Rust/WASM

### Backend
- **Rocket** - Web framework for Rust
- **Diesel** - ORM and Query Builder
- **PostgreSQL** - Database (in production)

## 🎮 Features

- Real-time dart scoring
- Player management
- Score tracking
- Checkout suggestions
- Game state persistence
- Browser warning on accidental page closure during active games

## 📝 License

[Add your license here]

## 🤝 Contributing

[Add contributing guidelines here]