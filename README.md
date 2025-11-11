# homedo

A web application built with Rust (Axum) backend and SvelteKit frontend.

## Tech Stack

- **Backend**: Rust with Axum web framework and Tokio async runtime
- **Frontend**: SvelteKit with TypeScript and Svelte 5

## Prerequisites

- Rust (edition 2024)
- Node.js and npm

## Development

### Backend

```bash
# Build the backend
cargo build

# Run the backend server
cargo run

# Run tests
cargo test
```

The backend server runs on `http://localhost:3001`.

### Frontend

```bash
# Navigate to frontend directory
cd static

# Install dependencies
npm install

# Run development server
npm run dev

# Build for production
npm run build

# Type check
npm run check
```

## Project Structure

```
homedo/
├── src/              # Rust backend source
├── static/           # SvelteKit frontend
│   ├── src/
│   │   ├── lib/     # Shared frontend code
│   │   └── routes/  # SvelteKit pages
│   └── static/      # Static assets
├── Cargo.toml       # Rust dependencies
└── package.json     # Frontend dependencies
```

## Production

1. Build the frontend: `cd static && npm run build`
2. Run the backend: `cargo run`

The Rust server serves the built frontend from `static/build` directory.
