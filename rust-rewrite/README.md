# Quepid Rust Rewrite

> **Status**: Phase 0 - Foundation (In Progress)

This is the Rust rewrite of Quepid, a search quality testing and improvement platform.

## Quick Start

### Prerequisites

- Rust 1.80+ (recommended: use [rustup](https://rustup.rs/))
- MySQL 8.0+ or PostgreSQL 14+ or SQLite
- Redis 7+ (for background jobs)

### Installation

```bash
# Clone the repository
git clone https://github.com/lauzel/quepid.git
cd quepid/rust-rewrite

# Build everything
cargo build --all --release

# Run the API server
cargo run --package quepid-api --release
```

### Using Docker

```bash
# Start the development environment
cd rust-rewrite/docker
docker compose up -d

# The API will be available at http://localhost:3000
```

## Project Structure

```
rust-rewrite/
├── Cargo.toml                    # Workspace manifest
├── rust-toolchain                # Toolchain version
├── crates/
│   ├── quepid-core/              # Domain layer
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── domain/
│   │   │   │   ├── entities/    # Domain entities
│   │   │   │   ├── value_objects/ # Value objects
│   │   │   │   ├── services/    # Domain services
│   │   │   │   └── errors.rs    # Domain errors
│   │   │   └── ports/           # Ports (interfaces)
│   │   └── Cargo.toml
│   │
│   ├── quepid-infra/             # Infrastructure layer
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── database/
│   │   │   │   ├── entities/    # SeaORM entities
│   │   │   │   ├── repositories/ # Repository implementations
│   │   │   │   └── connection.rs
│   │   │   ├── external/        # External service adapters
│   │   │   ├── jobs/            # Background job system
│   │   │   └── websocket/       # WebSocket server
│   │   └── Cargo.toml
│   │
│   ├── quepid-api/               # Presentation layer
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── main.rs
│   │   │   ├── app.rs           # Application setup
│   │   │   ├── config.rs        # Configuration
│   │   │   ├── routes/          # API routes
│   │   │   ├── handlers/        # Route handlers
│   │   │   └── middleware/      # Middleware
│   │   └── Cargo.toml
│   │
│   ├── quepid-workers/           # Background workers
│   │   └── src/
│   │       └── main.rs
│   │
│   ├── quepid-websocket/         # WebSocket server
│   │   └── src/
│   │       └── main.rs
│   │
│   └── quepid-cli/               # CLI utilities
│       └── src/
│           └── main.rs
│
├── docker/
│   ├── Dockerfile                # Production image
│   ├── Dockerfile.dev            # Development image
│   ├── docker-compose.yml        # Docker Compose config
│   └── config.toml               # Default configuration
│
├── migrations/                   # Database migrations
│
├── docs/                         # Documentation
│
├── scripts/                      # Utility scripts
│
└── README.md
```

## Architecture

The Rust rewrite follows **Clean Architecture** with **Hexagonal (Ports & Adapters)** pattern:

```
┌─────────────────────────────────────────────────────────┐
│                    Presentation Layer                        │
│  (quepid-api) Axum HTTP server, WebSocket, API routes      │
├─────────────────────────────────────────────────────────┤
│                    Application Layer                        │
│  (quepid-api) Use cases, authentication, authorization     │
├─────────────────────────────────────────────────────────┤
│                      Domain Layer                           │
│  (quepid-core) Entities, value objects, domain services   │
├─────────────────────────────────────────────────────────┤
│                    Infrastructure Layer                     │
│  (quepid-infra) Database, external services, jobs         │
└─────────────────────────────────────────────────────────┘
```

### Key Principles

1. **Dependency Rule**: Inner layers don't know about outer layers
   - Domain doesn't depend on Infrastructure or Presentation
   - Infrastructure depends on Domain, not vice versa
   - Presentation depends on both Domain and Infrastructure

2. **Interface Segregation**: Each port (interface) is focused on a single responsibility

3. **Testability**: Business logic can be tested without databases or HTTP

## Development

### Running Tests

```bash
# Run all tests
cargo test --all

# Run tests for a specific crate
cargo test --package quepid-core

# Run with coverage (requires cargo-tarpaulin)
cargo tarpaulin --all
```

### Running Linter

```bash
# Run clippy
cargo clippy --all -- -D warnings

# Run rustfmt
cargo fmt --all -- --check
```

### Running Benchmarks

```bash
# Run all benchmarks
cargo bench --all

# Run specific benchmark
cargo bench --package quepid-core
```

## Configuration

Configuration can be provided via:

1. **Environment variables** (prefixed with `QUEPID_`)
2. **Configuration file** (`config.toml`)
3. **Command-line arguments**

### Environment Variables

```bash
# Server
QUEPID_SERVER_HOST=0.0.0.0
QUEPID_SERVER_PORT=3000

# Database
QUEPID_DATABASE_BACKEND=mysql
QUEPID_DATABASE_CONNECTION_STRING=mysql://user:pass@host/db

# Authentication
QUEPID_AUTH_JWT_SECRET=your-secret-key
QUEPID_AUTH_JWT_EXPIRATION_HOURS=24

# Logging
RUST_LOG=debug
```

## Database Setup

### MySQL

```bash
# Create database
mysql -u root -p -e "CREATE DATABASE quepid_development;"

# Run migrations
cargo run --package quepid-cli -- migrate up
```

### PostgreSQL

```bash
# Create database
createdb quepid_development

# Run migrations
cargo run --package quepid-cli -- migrate up
```

### SQLite

```bash
# Database will be created automatically
cargo run --package quepid-api --release
```

## Migration from Ruby

### Database Migration

The Rust implementation uses SeaORM which can work with the existing database schema.

1. **Export schema from Ruby**:
   ```bash
   # In the Ruby project
   bundle exec rake db:schema:dump
   ```

2. **Generate SeaORM entities**:
   ```bash
   # Install sea-orm-cli
   cargo install sea-orm-cli
   
   # Generate entities from database
   sea-orm-cli generate entity -u mysql://user:pass@host/db -o crates/quepid-infra/src/database/entities
   ```

### API Compatibility

The Rust API aims to be 100% compatible with the Ruby API. Use the contract testing
framework to verify compatibility:

```bash
# Run contract tests
cargo test --package quepid-api --test contract
```

## Deployment

### Using Docker

```bash
# Build production image
cd rust-rewrite/docker
docker build -t quepid-rust -f Dockerfile .

# Run the container
docker run -d -p 3000:3000 \
  -e DATABASE_URL=mysql://user:pass@host/db \
  -e QUEPID_JWT_SECRET=your-secret \
  quepid-rust
```

### Using Docker Compose

```bash
cd rust-rewrite/docker
docker compose -f docker-compose.yml -f docker-compose.prod.yml up -d
```

## Monitoring

The API server exposes:

- **Health check**: `GET /healthcheck`
- **Metrics**: `GET /metrics` (if enabled)
- **OpenAPI docs**: `GET /api/docs` (if enabled)

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Code Style

- Run `cargo clippy --all -- -D warnings` before committing
- Run `cargo fmt --all` to format code
- Add tests for new functionality
- Update documentation as needed

## License

Apache 2.0 - See [LICENSE](../../LICENSE)

## Documentation

- [Full Rewrite Plan](../RUST_REWRITE_PLAN.md)
- [Roadmap](../rust-rewrite-roadmap.md)
- [Developer Guide](../../DEVELOPER_GUIDE.md)
- [App Structure](../../docs/app_structure.md)
- [Data Mapping](../../docs/data_mapping.md)
