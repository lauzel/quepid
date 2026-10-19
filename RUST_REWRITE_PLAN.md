# Quepid Rust Rewrite Plan

## Overview

This document outlines the comprehensive plan to rewrite Quepid from Ruby on Rails to Rust. The goal is to create a clean, performant, and maintainable Rust implementation that maintains full compatibility with the existing functionality while improving performance, type safety, and reliability.

## Project Context

Quepid is a search quality testing and improvement platform with:
- **203 Ruby controllers**
- **38+ models**
- **Complex data relationships** (Cases, Queries, Ratings, Scores, Books, Teams, etc.)
- **REST API** (versioned under `/api/v1`)
- **AngularJS frontend** (core app) + **Stimulus** (modern parts)
- **MySQL/PostgreSQL/SQLite** database support
- **Background jobs** (SolidQueue)
- **WebSocket support** (ActionCable)
- **Authentication** (Devise + OmniAuth for OIDC)
- **External integrations** (Solr, OpenSearch, LLM judges)

## Architecture Decision: Clean Architecture + Hexagonal

We'll use a **Clean Architecture** approach with **Hexagonal (Ports & Adapters)** pattern:

```
┌─────────────────────────────────────────────────────────┐
│                    Presentation Layer                        │
│  (HTTP API, WebSocket, GraphQL if needed)                  │
├─────────────────────────────────────────────────────────┤
│                    Application Layer                        │
│  (Use Cases, Domain Logic, Business Rules)                 │
├─────────────────────────────────────────────────────────┤
│                      Domain Layer                           │
│  (Entities, Value Objects, Domain Services)               │
├─────────────────────────────────────────────────────────┤
│                    Infrastructure Layer                     │
│  (Database, External Services, Message Brokers)            │
└─────────────────────────────────────────────────────────┘
```

### Technology Stack

| Component | Technology | Rationale |
|-----------|------------|-----------|
| **Web Framework** | Axum | Mature, well-documented, great ecosystem |
| **ORM** | SeaORM | Type-safe, async, supports multiple databases |
| **Database** | MySQL/PostgreSQL/SQLite | Same as current, via SeaORM |
| **Authentication** | custom JWT + OAuth2 | Replicate Devise + OmniAuth functionality |
| **Background Jobs** | custom worker pool + Redis | Replace SolidQueue |
| **WebSockets** | Tokio + tungstenite | Replace ActionCable |
| **API Documentation** | OpenAPI (utoipa) | Generate OpenAPI specs |
| **Testing** | Rust test + mockall + testcontainers | Comprehensive testing |
| **Frontend** | Keep existing (AngularJS/Stimulus) | Frontend stays the same, just API changes |
| **Configuration** | config-rs + serde | Type-safe configuration |
| **Logging** | tracing + slog | Structured logging |
| **Metrics** | metrics + prometheus | Observability |
| **Migration** | sea-orm-migration | Database migrations |

## Phase Roadmap

### Phase 0: Foundation (Week 1-2)
**Goal**: Set up Rust project structure, tooling, and CI

#### Tasks:
- [ ] 0.1 Create new Rust workspace with proper structure
- [ ] 0.2 Set up Cargo workspace with crates:
  - `quepid-core` - Domain models and business logic
  - `quepid-infra` - Database, external services
  - `quepid-api` - HTTP API (Axum)
  - `quepid-workers` - Background job processing
  - `quepid-websocket` - WebSocket server
  - `quepid-cli` - Command-line utilities
- [ ] 0.3 Configure CI/CD pipeline
  - GitHub Actions for Rust build, test, lint
  - Integration tests against current Ruby API
- [ ] 0.4 Set up development environment
  - Docker Compose for local development
  - Database containers (MySQL, PostgreSQL, SQLite)
  - Coding standards (clippy, rustfmt)
- [ ] 0.5 Create OpenAPI specification
  - Extract from existing Rails routes
  - Use utoipa to generate Rust docs from code

#### Deliverables:
- Working Rust project structure
- CI pipeline running
- OpenAPI spec v1

---

### Phase 1: Domain Layer (Week 3-4)
**Goal**: Implement core domain models and business logic

#### Tasks:
- [ ] 1.1 Define domain entities (Rust structs with validation)
  - User, Team, Case, Query, Rating, Score
  - Scorer, Try, SearchEndpoint, Snapshot
  - Book, QueryDocPair, Judgement
  - Annotation, ApiKey, Announcement
  - All remaining models from `app/models/`
- [ ] 1.2 Implement value objects
  - EmailAddress, PasswordHash, CaseName, etc.
- [ ] 1.3 Define domain services
  - RatingCalculator, ScoreCalculator
  - SnapshotService, ImportService, ExportService
  - JudgementAggregator
- [ ] 1.4 Implement domain errors
  - Custom error types for each domain
  - thiserror + anyhow for error handling
- [ ] 1.5 Create domain tests
  - Unit tests for all domain logic
  - Property-based tests where applicable

#### Deliverables:
- `quepid-core` crate with all domain models
- Comprehensive domain test suite

---

### Phase 2: Infrastructure Layer (Week 5-8)
**Goal**: Implement database access and external service integrations

#### Tasks:

**Database (SeaORM):**
- [ ] 2.1 Define SeaORM entities for all models
- [ ] 2.2 Implement repository pattern
  - UserRepository, CaseRepository, etc.
- [ ] 2.3 Implement database migrations
  - Migrate from current schema
  - SeaORM migration generator
- [ ] 2.4 Implement connection pooling
- [ ] 2.5 Support multiple database backends
  - MySQL, PostgreSQL, SQLite

**External Services:**
- [ ] 2.6 Search Engine Adapters
  - Solr adapter
  - OpenSearch adapter
  - Generic SearchAPI adapter
- [ ] 2.7 LLM Judge Adapters
  - OpenAI, Anthropic, Google, etc.
- [ ] 2.8 Email service (Postmark)
- [ ] 2.9 Analytics (Ahoy replacement)

**Background Jobs:**
- [ ] 2.10 Worker pool implementation
- [ ] 2.11 Job queue (Redis-based)
- [ ] 2.12 Job types:
  - ExportCaseJob
  - ImportCaseJob
  - RunAiJudgeJob
  - CalculateScoresJob
  - etc.

**WebSockets:**
- [ ] 2.13 WebSocket server (tungstenite)
- [ ] 2.14 Connection management
- [ ] 2.15 Message routing

#### Deliverables:
- `quepid-infra` crate with all adapters
- Database migrations
- Background job system
- WebSocket server

---

### Phase 3: Application Layer (Week 9-12)
**Goal**: Implement use cases and application services

#### Tasks:

**Authentication & Authorization:**
- [ ] 3.1 JWT token management
- [ ] 3.2 Password hashing (bcrypt/argon2)
- [ ] 3.3 Session management
- [ ] 3.4 OAuth2/OIDC integration
  - Keycloak, Google, OpenID Connect
- [ ] 3.5 Authorization middleware
  - Role-based access control
  - Team membership checks

**API Use Cases:**
- [ ] 3.6 User management
  - Registration, login, profile, password reset
- [ ] 3.7 Case management
  - CRUD, cloning, archiving
- [ ] 3.8 Query management
- [ ] 3.9 Rating management
- [ ] 3.10 Score calculation
- [ ] 3.11 Snapshot management
- [ ] 3.12 Book management
- [ ] 3.13 Judgement management
- [ ] 3.14 Scorer management
- [ ] 3.15 Search endpoint management
- [ ] 3.16 Team management
- [ ] 3.17 AI Judge management
- [ ] 3.18 Admin operations

**Real-time Features:**
- [ ] 3.19 WebSocket message handlers
- [ ] 3.20 Progress notifications
- [ ] 3.21 Job status updates

#### Deliverables:
- `quepid-api` crate with all use cases
- Complete authentication system

---

### Phase 4: Presentation Layer (Week 13-14)
**Goal**: Implement HTTP API and WebSocket endpoints

#### Tasks:

**HTTP API (Axum):**
- [ ] 4.1 Route definitions (match current routes.rb)
- [ ] 4.2 Request validation
- [ ] 4.3 Response formatting (JSON:API or custom)
- [ ] 4.4 Error handling middleware
- [ ] 4.5 Request/response logging
- [ ] 4.6 Rate limiting
- [ ] 4.7 CORS configuration
- [ ] 4.8 Health check endpoints

**API Endpoints:**
- [ ] 4.9 Auth endpoints (/users/login, /users/signup, etc.)
- [ ] 4.10 Case endpoints (/api/v1/cases, /cases, etc.)
- [ ] 4.11 Query endpoints
- [ ] 4.12 Rating endpoints
- [ ] 4.13 Score endpoints
- [ ] 4.14 Book endpoints
- [ ] 4.15 Judgement endpoints
- [ ] 4.16 Scorer endpoints
- [ ] 4.17 Search endpoint endpoints
- [ ] 4.18 AI Judge endpoints
- [ ] 4.19 Team endpoints
- [ ] 4.20 Admin endpoints
- [ ] 4.21 Proxy endpoints
- [ ] 4.22 Dropdown endpoints
- [ ] 4.23 Mapper wizard endpoints
- [ ] 4.24 Bulk judge endpoints

**WebSocket:**
- [ ] 4.25 WebSocket route definitions
- [ ] 4.26 Message serialization/deserialization

#### Deliverables:
- Complete HTTP API server
- WebSocket server integration

---

### Phase 5: Testing & Validation (Week 15-16)
**Goal**: Ensure Rust implementation matches Ruby behavior

#### Tasks:

**Testing Strategy:**
- [ ] 5.1 Integration tests against staging Ruby API
- [ ] 5.2 Property-based testing for domain logic
- [ ] 5.3 Contract testing (Pact or custom)
- [ ] 5.4 Performance benchmarking
- [ ] 5.5 Load testing

**Comparison Testing:**
- [ ] 5.6 Automated comparison tool
  - Capture Ruby API responses
  - Compare with Rust API responses
  - Flag discrepancies
- [ ] 5.7 Test data generation
  - Export production data (anonymized)
  - Generate synthetic test data
- [ ] 5.8 Edge case testing
  - Concurrent requests
  - Database connection issues
  - Invalid inputs

**CI/CD:**
- [ ] 5.9 Parallel test execution
- [ ] 5.10 Test coverage reporting
- [ ] 5.11 Performance regression tests

#### Deliverables:
- Comprehensive test suite
- Comparison testing framework
- Performance benchmarks

---

### Phase 6: Migration & Deployment (Week 17-18)
**Goal**: Deploy Rust implementation alongside Ruby for gradual migration

#### Tasks:

**Deployment Strategy:**
- [ ] 6.1 Blue-green deployment setup
- [ ] 6.2 Database migration scripts
- [ ] 6.3 Configuration migration
- [ ] 6.4 Feature flags for gradual rollout

**Monitoring:**
- [ ] 6.5 Metrics collection
  - Request rates, error rates, latencies
  - Database query performance
  - Background job metrics
- [ ] 6.6 Alerting setup
- [ ] 6.7 Distributed tracing

**Documentation:**
- [ ] 6.8 Rust API documentation
- [ ] 6.9 Migration guide
- [ ] 6.10 Deployment runbooks

#### Deliverables:
- Production-ready deployment
- Monitoring and alerting
- Migration documentation

---

### Phase 7: Frontend Integration (Week 19-20)
**Goal**: Update frontend to work with Rust API

#### Tasks:

**Frontend Changes:**
- [ ] 7.1 Update API client configuration
- [ ] 7.2 Handle any API response format differences
- [ ] 7.3 Update WebSocket client
- [ ] 7.4 Fix any CORS issues
- [ ] 7.5 Performance optimizations

**Testing:**
- [ ] 7.6 End-to-end tests with new backend
- [ ] 7.7 Browser compatibility testing
- [ ] 7.8 Mobile responsiveness testing

#### Deliverables:
- Frontend working with Rust backend
- Updated tests

---

### Phase 8: Cleanup & Optimization (Week 21-22)
**Goal**: Final polish and performance optimization

#### Tasks:

**Performance:**
- [ ] 8.1 Database query optimization
- [ ] 8.2 Caching implementation
- [ ] 8.3 Connection pooling tuning
- [ ] 8.4 Memory usage optimization
- [ ] 8.5 CPU profiling and optimization

**Code Quality:**
- [ ] 8.6 Clippy warnings cleanup
- [ ] 8.7 Documentation completion
- [ ] 8.8 Code coverage improvement
- [ ] 8.9 Dependency updates

**Final Testing:**
- [ ] 8.10 Security audit
- [ ] 8.11 Penetration testing
- [ ] 8.12 Chaos engineering tests

#### Deliverables:
- Optimized production code
- Complete documentation

---

## Testing Strategy

### 1. Unit Testing
- Domain models: Property-based tests with proptest
- Services: Mock external dependencies with mockall
- Repositories: Test with real database (testcontainers)

### 2. Integration Testing
- API endpoints: Test with real database
- Use cases: Test complete flows
- External services: Mock or use test instances

### 3. Contract Testing
- **Automated comparison with Ruby API**
- Capture Ruby responses for known inputs
- Compare Rust responses against Ruby baseline
- Flag any discrepancies for investigation

```rust
// Example comparison test
#[tokio::test]
async fn test_case_list_matches_ruby() {
    // Setup test data
    let test_data = setup_test_cases();
    
    // Get Ruby response (from staging or recorded)
    let ruby_response = get_ruby_response("/api/v1/cases");
    
    // Get Rust response
    let rust_response = get_rust_response("/api/v1/cases");
    
    // Compare responses
    assert_json_matches(&ruby_response, &rust_response);
}
```

### 4. Property-Based Testing
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn score_calculation_is_deterministic(
        ratings in collection::vec((0i32..5i32), 1..100),
    ) {
        let score1 = calculate_score(&ratings);
        let score2 = calculate_score(&ratings);
        
        prop_assert_eq!(score1, score2);
    }
    
    #[test]
    fn score_is_between_0_and_100(
        ratings in collection::vec((0i32..5i32), 1..100),
    ) {
        let score = calculate_score(&ratings);
        
        prop_assert!(score >= 0.0 && score <= 100.0);
    }
}
```

### 5. Performance Testing
- Benchmark critical paths
- Compare with Ruby implementation
- Set performance regression thresholds

## File Structure

```
quepid-rust/
├── Cargo.toml                    # Workspace manifest
├── rust-toolchain                # Toolchain version
├── .github/
│   └── workflows/
│       ├── ci.yml               # CI pipeline
│       ├── benchmark.yml        # Performance tests
│       └── deploy.yml            # Deployment
├── crates/
│   ├── quepid-core/              # Domain layer
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── domain/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── entities/    # Domain entities
│   │   │   │   │   ├── user.rs
│   │   │   │   │   ├── case.rs
│   │   │   │   │   ├── query.rs
│   │   │   │   │   └── ...
│   │   │   │   ├── value_objects/ # Value objects
│   │   │   │   ├── services/    # Domain services
│   │   │   │   └── errors.rs    # Domain errors
│   │   │   └── tests/
│   │   │       ├── domain_tests.rs
│   │   │       └── ...
│   │   └── benches/
│   │       └── benchmark.rs
│   │
│   ├── quepid-infra/              # Infrastructure layer
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── database/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── entities/    # SeaORM entities
│   │   │   │   │   └── mod.rs (generated)
│   │   │   │   ├── repositories/ # Repository implementations
│   │   │   │   │   ├── user_repository.rs
│   │   │   │   │   ├── case_repository.rs
│   │   │   │   │   └── ...
│   │   │   │   └── migrations/  # Database migrations
│   │   │   ├── external/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── search_engines/
│   │   │   │   │   ├── solr.rs
│   │   │   │   │   ├── opensearch.rs
│   │   │   │   │   └── mod.rs
│   │   │   │   ├── llm/
│   │   │   │   │   ├── openai.rs
│   │   │   │   │   ├── anthropic.rs
│   │   │   │   │   └── mod.rs
│   │   │   │   └── email.rs
│   │   │   ├── jobs/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── worker.rs
│   │   │   │   ├── job_types.rs
│   │   │   │   └── handlers/
│   │   │   └── websocket/
│   │   │       ├── mod.rs
│   │   │       ├── server.rs
│   │   │       └── message.rs
│   │   └── tests/
│   │
│   ├── quepid-api/                # Presentation layer
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── main.rs
│   │   │   ├── config/
│   │   │   │   ├── mod.rs
│   │   │   │   └── settings.rs
│   │   │   ├── routes/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── api/
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── v1/
│   │   │   │   │   │   ├── mod.rs
│   │   │   │   │   │   ├── cases.rs
│   │   │   │   │   │   ├── queries.rs
│   │   │   │   │   │   ├── ratings.rs
│   │   │   │   │   │   ├── scores.rs
│   │   │   │   │   │   ├── books.rs
│   │   │   │   │   │   ├── judgements.rs
│   │   │   │   │   │   ├── scorers.rs
│   │   │   │   │   │   ├── search_endpoints.rs
│   │   │   │   │   │   ├── ai_judges.rs
│   │   │   │   │   │   ├── teams.rs
│   │   │   │   │   │   └── users.rs
│   │   │   │   │   └── web/
│   │   │   │   │       ├── mod.rs
│   │   │   │   │       ├── sessions.rs
│   │   │   │   │       ├── accounts.rs
│   │   │   │   │       ├── pages.rs
│   │   │   │   │       ├── admin.rs
│   │   │   │   │       └── ...
│   │   │   ├── middleware/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── auth.rs
│   │   │   │   ├── error_handler.rs
│   │   │   │   ├── logging.rs
│   │   │   │   └── rate_limiter.rs
│   │   │   ├── handlers/
│   │   │   │   ├── mod.rs
│   │   │   │   └── ...
│   │   │   └── app.rs
│   │   └── tests/
│   │       ├── integration/
│   │       └── ...
│   │
│   ├── quepid-workers/            # Background workers
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs
│   │       └── ...
│   │
│   └── quepid-cli/                # CLI utilities
│       ├── Cargo.toml
│       └── src/
│           └── main.rs
│
├── docker/
│   ├── Dockerfile                # Production image
│   ├── Dockerfile.dev            # Development image
│   └── docker-compose.yml
│
├── docs/
│   ├── architecture.md
│   ├── api/
│   │   └── openapi.yaml
│   └── migration_guide.md
│
├── scripts/
│   ├── setup.sh
│   ├── test_comparison.sh
│   └── benchmark.sh
│
├── tests/
│   ├── integration/
│   ├── contract/
│   └── performance/
│
└── README.md
```

## Dependency Management

```toml
# quepid-core/Cargo.toml
[package]
name = "quepid-core"
version = "0.1.0"
edition = "2021"

[dependencies]
# Domain
thiserror = "2.0"
anyhow = "1.0"
validator = { version = "2.0", features = ["derive"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1.0", features = ["v4", "serde"] }

# Testing
mockall = "0.12"
proptest = "0.11"

[dev-dependencies]
test-case = "3.0"
```

```toml
# quepid-infra/Cargo.toml
[package]
name = "quepid-infra"
version = "0.1.0"
edition = "2021"

[dependencies]
# Core
quepid-core = { path = "../quepid-core" }

# Database
sea-orm = { version = "1.0", features = ["sqlx-mysql", "sqlx-postgres", "sqlx-sqlite", "runtime-tokio"] }
sea-orm-migration = "1.0"

# Async
tokio = { version = "1.0", features = ["full"] }
async-trait = "0.1"

# HTTP
reqwest = { version = "0.12", features = ["json"] }

# Redis
redis = { version = "0.25", features = ["tokio-comp"] }

# WebSocket
tokio-tungstenite = "0.20"

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Logging
tracing = "0.1"

# External services
openai-api-rs = "0.1"  # or custom implementation

[dev-dependencies]
mockall = "0.12"
testcontainers = "0.15"
```

```toml
# quepid-api/Cargo.toml
[package]
name = "quepid-api"
version = "0.1.0"
edition = "2021"

[dependencies]
# Core
quepid-core = { path = "../quepid-core" }
quepid-infra = { path = "../quepid-infra" }

# Web framework
axum = "0.7"
axum-extra = "0.9"
tower = "0.4"
tower-http = { version = "0.5", features = ["cors", "rate-limit", "trace"] }

# Authentication
jsonwebtoken = "9.0"
bcrypt = "0.15"
argon2 = "0.5"

# Validation
validator = { version = "2.0", features = ["derive"] }

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# OpenAPI
utoipa = { version = "4.0", features = ["axum_extras", "chrono", "uuid"] }
utoipa-swagger-ui = "4.0"

# Configuration
config = "0.13"
serde = { version = "1.0", features = ["derive"] }

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

# Metrics
metrics = "0.22"
metrics-exporter-prometheus = "0.12"

[dev-dependencies]
# ...
```

## Database Schema Migration

We'll use SeaORM's migration system to maintain the database schema:

```rust
// migrations/20240101000000_create_users.rs
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(User::Id).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(User::Email).string().not_null().unique_key())
                    .col(ColumnDef::new(User::Name).string().not_null())
                    .col(ColumnDef::new(User::PasswordDigest).string().not_null())
                    .col(ColumnDef::new(User::Admin).boolean().not_null().default_value(false))
                    .col(ColumnDef::new(User::CreatedAt).timestamp().not_null().default_value(Expr::current_timestamp()))
                    .col(ColumnDef::new(User::UpdatedAt).timestamp().not_null().default_value(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(User::Table).to_owned()).await
    }
}
```

## API Endpoint Examples

### User Authentication

```rust
// quepid-api/src/routes/web/sessions.rs
use axum::{Json, Router, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use quepid_core::domain::entities::user::User;
use quepid_infra::database::repositories::UserRepository;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i64,
    pub email: String,
    pub name: String,
    pub admin: bool,
}

pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    let user = state
        .user_repository
        .find_by_email(&request.email)
        .await
        .ok_or(AppError::InvalidCredentials)?;
    
    if !user.verify_password(&request.password) {
        return Err(AppError::InvalidCredentials);
    }
    
    let token = state.auth_service.generate_token(user.id)?;
    
    Ok(Json(LoginResponse {
        token,
        user: UserResponse::from(user),
    }))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/login", axum::routing::post(login))
        .route("/logout", axum::routing::post(logout))
}
```

### Case Management

```rust
// quepid-api/src/routes/api/v1/cases.rs
use axum::{Json, Router, extract::{State, Path, Query}};
use quepid_core::domain::{
    entities::case::{Case, CreateCase, UpdateCase},
    services::case_service::CaseService,
};

#[derive(Debug, Deserialize)]
pub struct CreateCaseRequest {
    pub name: String,
    pub description: Option<String>,
    pub search_endpoint_id: Option<i64>,
    pub is_public: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCaseRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub search_endpoint_id: Option<i64>,
    pub is_public: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ListCasesQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub search: Option<String>,
}

pub async fn list_cases(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Query(query): Query<ListCasesQuery>,
) -> Result<Json<PaginatedResponse<CaseResponse>>, AppError> {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(20);
    
    let (cases, total) = state
        .case_service
        .list_by_user(auth.user_id, page, per_page, query.search)
        .await?;
    
    Ok(Json(PaginatedResponse {
        data: cases.into_iter().map(CaseResponse::from).collect(),
        meta: PaginationMeta {
            page,
            per_page,
            total,
            total_pages: (total as f64 / per_page as f64).ceil() as i64,
        },
    }))
}

pub async fn create_case(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Json(request): Json<CreateCaseRequest>,
) -> Result<Json<CaseResponse>, AppError> {
    let create_case = CreateCase {
        name: request.name,
        description: request.description,
        search_endpoint_id: request.search_endpoint_id,
        is_public: request.is_public,
        owner_id: auth.user_id,
    };
    
    let case = state.case_service.create(create_case).await?;
    
    Ok(Json(CaseResponse::from(case)))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/cases", axum::routing::get(list_cases))
        .route("/cases", axum::routing::post(create_case))
        .route("/cases/:id", axum::routing::get(get_case))
        .route("/cases/:id", axum::routing::put(update_case))
        .route("/cases/:id", axum::routing::delete(delete_case))
        .route("/cases/:id/clone", axum::routing::post(clone_case))
}
```

## Background Job System

```rust
// quepid-infra/src/jobs/mod.rs
use std::sync::Arc;
use tokio::sync::mpsc;
use quepid_core::domain::entities::Job;

pub enum JobType {
    ExportCase(ExportCaseJob),
    ImportCase(ImportCaseJob),
    RunAiJudge(RunAiJudgeJob),
    CalculateScores(CalculateScoresJob),
    // ... other job types
}

pub struct JobQueue {
    sender: mpsc::Sender<JobType>,
    receiver: Arc<Mutex<mpsc::Receiver<JobType>>>,
    workers: Vec<WorkerHandle>,
}

impl JobQueue {
    pub fn new(num_workers: usize) -> Self {
        let (sender, receiver) = mpsc::channel(100);
        let receiver = Arc::new(Mutex::new(receiver));
        
        let mut workers = Vec::new();
        for _ in 0..num_workers {
            let receiver_clone = receiver.clone();
            let handle = tokio::spawn(async move {
                Self::worker_loop(receiver_clone).await;
            });
            workers.push(WorkerHandle(handle));
        }
        
        Self { sender, receiver, workers }
    }
    
    pub async fn enqueue(&self, job: JobType) -> Result<(), JobError> {
        self.sender.send(job).await
            .map_err(|_| JobError::QueueFull)
    }
    
    async fn worker_loop(receiver: Arc<Mutex<mpsc::Receiver<JobType>>>) {
        loop {
            let job = receiver.lock().await.recv().await;
            match job {
                Some(job) => {
                    if let Err(e) = Self::process_job(job).await {
                        log::error!("Job processing error: {}", e);
                    }
                }
                None => break,
            }
        }
    }
    
    async fn process_job(job: JobType) -> Result<(), JobError> {
        match job {
            JobType::ExportCase(j) => j.execute().await,
            JobType::ImportCase(j) => j.execute().await,
            JobType::RunAiJudge(j) => j.execute().await,
            JobType::CalculateScores(j) => j.execute().await,
        }
    }
}
```

## WebSocket Implementation

```rust
// quepid-infra/src/websocket/server.rs
use axum::extract::ws::{WebSocket, WebSocketUpgrade};
use futures::{SinkExt, StreamExt};
use tokio::sync::broadcast;
use std::collections::HashMap;

pub struct WebSocketServer {
    clients: Arc<Mutex<HashMap<String, broadcast::Sender<Message>>>>,
}

impl WebSocketServer {
    pub fn new() -> Self {
        Self {
            clients: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    pub async fn handle_connection(
        &self,
        ws: WebSocketUpgrade,
        user_id: String,
    ) -> Result<(), WebSocketError> {
        let (mut sender, mut receiver) = ws.on_upgrade().await?;
        
        let (tx, mut rx) = broadcast::channel(100);
        
        {
            let mut clients = self.clients.lock().await;
            clients.insert(user_id.clone(), tx);
        }
        
        // Send welcome message
        sender.send(Message::Text("{"type":"connected"}".to_string())).await?;
        
        // Handle incoming messages
        while let Some(msg) = receiver.next().await {
            match msg? {
                Message::Text(text) => {
                    self.handle_message(&user_id, &text).await?;
                }
                Message::Close(_) => {
                    let mut clients = self.clients.lock().await;
                    clients.remove(&user_id);
                    break;
                }
                _ => {}
            }
        }
        
        Ok(())
    }
    
    pub async fn broadcast(&self, message: Message, except: Option<&str>) {
        let clients = self.clients.lock().await;
        for (user_id, tx) in clients.iter() {
            if Some(user_id.as_str()) != except {
                let _ = tx.send(message.clone());
            }
        }
    }
}
```

## Error Handling

```rust
// quepid-core/src/domain/errors.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    
    #[error("Forbidden: {0}")]
    Forbidden(String),
    
    #[error("Conflict: {0}")]
    Conflict(String),
    
    #[error("Internal error: {0}")]
    InternalError(String),
}

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sea_orm::DbErr),
    
    #[error("Not found")]
    NotFound,
    
    #[error("Duplicate entry")]
    DuplicateEntry,
}

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Domain error: {0}")]
    DomainError(#[from] DomainError),
    
    #[error("Repository error: {0}")]
    RepositoryError(#[from] RepositoryError),
    
    #[error("External service error: {0}")]
    ExternalServiceError(String),
}

// quepid-api/src/error.rs
use axum::{
    response::{IntoResponse, Response},
    http::StatusCode,
};

#[derive(Debug)]
pub enum AppError {
    Domain(DomainError),
    Repository(RepositoryError),
    Service(ServiceError),
    Authentication(AuthenticationError),
    Validation(ValidationError),
    NotFound,
    InternalServerError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Domain(e) => (StatusCode::BAD_REQUEST, e.to_string()),
            AppError::Repository(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            AppError::Service(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            AppError::Authentication(e) => (StatusCode::UNAUTHORIZED, e.to_string()),
            AppError::Validation(e) => (StatusCode::UNPROCESSABLE_ENTITY, e.to_string()),
            AppError::NotFound => (StatusCode::NOT_FOUND, "Not found".to_string()),
            AppError::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string()),
        };
        
        (status, Json(serde_json::json!({
            "error": error_message,
        }))).into_response()
    }
}
```

## Comparison Testing Framework

```rust
// tests/contract/mod.rs
use reqwest::Client;
use serde_json::Value;
use std::fs;

pub struct ContractTester {
    ruby_api_url: String,
    rust_api_url: String,
    client: Client,
}

impl ContractTester {
    pub fn new(ruby_api_url: String, rust_api_url: String) -> Self {
        Self {
            ruby_api_url,
            rust_api_url,
            client: Client::new(),
        }
    }
    
    pub async fn compare_endpoint(
        &self,
        path: &str,
        method: &str,
        body: Option<Value>,
        headers: Vec<(&str, &str)>,
    ) -> Result<(), ContractTestError> {
        // Get Ruby response
        let ruby_response = self.request(&self.ruby_api_url, path, method, body.clone(), &headers).await?;
        
        // Get Rust response
        let rust_response = self.request(&self.rust_api_url, path, method, body, &headers).await?;
        
        // Compare responses
        self.compare_responses(&ruby_response, &rust_response, path)?;
        
        Ok(())
    }
    
    pub async fn record_ruby_baseline(
        &self,
        path: &str,
        method: &str,
        body: Option<Value>,
        headers: Vec<(&str, &str)>,
        output_path: &str,
    ) -> Result<(), ContractTestError> {
        let response = self.request(&self.ruby_api_url, path, method, body, &headers).await?;
        
        let output = serde_json::json!({
            "path": path,
            "method": method,
            "status": response.status.as_u16(),
            "body": response.body,
            "headers": response.headers,
        });
        
        fs::write(output_path, serde_json::to_string_pretty(&output)?)?;
        
        Ok(())
    }
    
    pub async fn compare_with_baseline(
        &self,
        path: &str,
        method: &str,
        body: Option<Value>,
        headers: Vec<(&str, &str)>,
        baseline_path: &str,
    ) -> Result<(), ContractTestError> {
        let baseline: Value = serde_json::from_str(&fs::read_to_string(baseline_path)?)?;
        let rust_response = self.request(&self.rust_api_url, path, method, body, &headers).await?;
        
        let rust_output = serde_json::json!({
            "path": path,
            "method": method,
            "status": rust_response.status.as_u16(),
            "body": rust_response.body,
            "headers": rust_response.headers,
        });
        
        self.compare_responses(&baseline, &rust_output, path)?;
        
        Ok(())
    }
    
    fn compare_responses(
        &self,
        expected: &Value,
        actual: &Value,
        path: &str,
    ) -> Result<(), ContractTestError> {
        // Compare status codes
        let expected_status = expected["status"].as_u64().ok_or_else(|| 
            ContractTestError::InvalidBaseline(path.to_string()))? as u16;
        let actual_status = actual["status"].as_u64().ok_or_else(|| 
            ContractTestError::InvalidResponse(path.to_string()))? as u16;
        
        if expected_status != actual_status {
            return Err(ContractTestError::StatusMismatch {
                path: path.to_string(),
                expected: expected_status,
                actual: actual_status,
            });
        }
        
        // Compare body (with flexible comparison for non-deterministic fields)
        let expected_body = &expected["body"];
        let actual_body = &actual["body"];
        
        self.compare_bodies(expected_body, actual_body, path)?;
        
        Ok(())
    }
    
    fn compare_bodies(
        &self,
        expected: &Value,
        actual: &Value,
        path: &str,
    ) -> Result<(), ContractTestError> {
        // Implement flexible comparison that ignores:
        // - Timestamps (compare as dates, not exact times)
        // - IDs (if newly created)
        // - Non-deterministic order in arrays
        // - etc.
        
        // For now, simple deep equality
        if expected != actual {
            return Err(ContractTestError::BodyMismatch {
                path: path.to_string(),
                expected: expected.clone(),
                actual: actual.clone(),
            });
        }
        
        Ok(())
    }
}
```

## Performance Benchmarking

```rust
// benches/benchmark.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use quepid_core::domain::services::score_calculator::ScoreCalculator;

fn benchmark_score_calculation(c: &mut Criterion) {
    let mut group = c.benchmark_group("Score Calculation");
    
    // Small case (10 queries)
    let small_ratings = vec![vec![5, 4, 3, 2, 1]; 10];
    group.bench_with_input(
        BenchmarkId::new("Small Case", "10 queries"),
        &small_ratings,
        |b, ratings| {
            b.iter(|| {
                let scores = ScoreCalculator::calculate(black_box(ratings));
                black_box(scores)
            })
        },
    );
    
    // Medium case (100 queries)
    let medium_ratings = vec![vec![5, 4, 3, 2, 1]; 100];
    group.bench_with_input(
        BenchmarkId::new("Medium Case", "100 queries"),
        &medium_ratings,
        |b, ratings| {
            b.iter(|| {
                let scores = ScoreCalculator::calculate(black_box(ratings));
                black_box(scores)
            })
        },
    );
    
    // Large case (1000 queries)
    let large_ratings = vec![vec![5, 4, 3, 2, 1]; 1000];
    group.bench_with_input(
        BenchmarkId::new("Large Case", "1000 queries"),
        &large_ratings,
        |b, ratings| {
            b.iter(|| {
                let scores = ScoreCalculator::calculate(black_box(ratings));
                black_box(scores)
            })
        },
    );
    
    group.finish();
}

criterion_group!(benches, benchmark_score_calculation);
criterion_main!(benches);
```

## CI/CD Pipeline

```yaml
# .github/workflows/ci.yml
name: CI

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true
          components: rustfmt, clippy
      
      - name: Cache dependencies
        uses: actions/cache@v3
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
      
      - name: Build
        run: cargo build --all --release
      
      - name: Run clippy
        run: cargo clippy --all -- -D warnings
      
      - name: Run rustfmt
        run: cargo fmt --all -- --check

  test:
    needs: build
    runs-on: ubuntu-latest
    services:
      mysql:
        image: mysql:8.0
        env:
          MYSQL_ROOT_PASSWORD: password
          MYSQL_DATABASE: quepid_test
        ports:
          - 3306:3306
        options: --health-cmd="mysqladmin ping" --health-interval=10s --health-timeout=5s --health-retries=3
      
      postgres:
        image: postgres:14
        env:
          POSTGRES_PASSWORD: password
          POSTGRES_DB: quepid_test
        ports:
          - 5432:5432
        options: --health-cmd pg_isready --health-interval 10s --health-timeout 5s --health-retries 3
      
      redis:
        image: redis:7
        ports:
          - 6379:6379
        options: --health-cmd="redis-cli ping" --health-interval=10s --health-timeout=5s --health-retries=3
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Run unit tests
        run: cargo test --all -- --test-threads=1
      
      - name: Run integration tests (MySQL)
        env:
          DATABASE_URL: mysql://root:password@localhost:3306/quepid_test
        run: cargo test --package quepid-infra --test integration_mysql
      
      - name: Run integration tests (PostgreSQL)
        env:
          DATABASE_URL: postgresql://postgres:password@localhost:5432/quepid_test
        run: cargo test --package quepid-infra --test integration_postgres
      
      - name: Run contract tests
        env:
          RUBY_API_URL: https://staging.quepid.com
          RUST_API_URL: http://localhost:3000
        run: cargo test --package quepid-api --test contract

  contract-test:
    needs: build
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Start Rust API
        run: |
          cargo run --package quepid-api --release &
          sleep 10
      
      - name: Run contract tests
        run: cargo test --package quepid-api --test contract

  benchmark:
    needs: build
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Run benchmarks
        run: cargo bench --all
      
      - name: Compare with previous benchmarks
        uses: actions/cache@v3
        with:
          path: criterion-results
          key: benchmark-${{ github.sha }}
        
        # Compare and fail if performance regressed by more than 5%
        run: |
          git clone https://github.com/${{ github.repository }}.git previous
          cd previous
          git checkout ${{ github.base_ref }}
          cargo bench --all
          
          # Compare results
          python3 scripts/compare_benchmarks.py --current ../criterion-results --previous criterion-results --threshold 0.05

  deploy-staging:
    needs: [test, contract-test]
    if: github.ref == 'refs/heads/develop'
    runs-on: ubuntu-latest
    environment:
      name: staging
      url: https://staging-rust.quepid.com
    steps:
      - uses: actions/checkout@v4
      
      - name: Build Docker image
        run: docker build -t quepid-rust:staging -f docker/Dockerfile .
      
      - name: Deploy to staging
        run: |
          docker push quepid-rust:staging
          # Deployment commands for staging

  deploy-production:
    needs: [deploy-staging]
    if: github.ref == 'refs/heads/main'
    runs-on: ubuntu-latest
    environment:
      name: production
      url: https://quepid.com
    steps:
      - uses: actions/checkout@v4
      
      - name: Build Docker image
        run: docker build -t quepid-rust:production -f docker/Dockerfile .
      
      - name: Deploy to production
        run: |
          docker push quepid-rust:production
          # Deployment commands for production
```

## Timeline & Milestones

| Phase | Duration | Start Date | End Date | Key Deliverable |
|-------|----------|------------|----------|-----------------|
| Phase 0: Foundation | 2 weeks | Week 1 | Week 2 | Project structure, CI, OpenAPI |
| Phase 1: Domain Layer | 2 weeks | Week 3 | Week 4 | Domain models & tests |
| Phase 2: Infrastructure | 4 weeks | Week 5 | Week 8 | Database, jobs, WebSocket |
| Phase 3: Application | 4 weeks | Week 9 | Week 12 | Use cases & auth |
| Phase 4: Presentation | 2 weeks | Week 13 | Week 14 | HTTP API & WebSocket endpoints |
| Phase 5: Testing | 2 weeks | Week 15 | Week 16 | Contract tests & benchmarks |
| Phase 6: Migration | 2 weeks | Week 17 | Week 18 | Deployment & monitoring |
| Phase 7: Frontend | 2 weeks | Week 19 | Week 20 | Frontend integration |
| Phase 8: Cleanup | 2 weeks | Week 21 | Week 22 | Optimization & documentation |

**Total Estimated Duration: 22 weeks (5.5 months)**

## Team Structure

For optimal progress, we recommend the following team structure:

### Core Team (3-4 developers)
- **Tech Lead / Architect**: Oversees architecture decisions, code reviews
- **Backend Developer 1**: Focuses on domain and application layers
- **Backend Developer 2**: Focuses on infrastructure and presentation layers
- **DevOps Engineer**: CI/CD, deployment, monitoring

### Extended Team (Optional)
- **Frontend Developer**: Helps with frontend integration
- **QA Engineer**: Focuses on testing and validation
- **Technical Writer**: Documentation

## Risk Mitigation

### Technical Risks

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| Performance issues with Rust | Low | High | Benchmark early, optimize critical paths |
| Database compatibility issues | Medium | High | Test with all 3 databases, use SeaORM abstractions |
| Frontend integration issues | Medium | Medium | Keep API contracts identical, test early |
| Missing Rust crates for functionality | Low | Medium | Use custom implementations, contribute to OSS |
| Memory safety issues | Low | High | Use Rust's type system, extensive testing |

### Project Risks

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| Scope creep | Medium | High | Strict scope management, MVP first |
| Team availability | Medium | High | Cross-train team members, documentation |
| Dependency on external services | Medium | Medium | Mock external services, contract tests |
| Migration downtime | Low | High | Blue-green deployment, thorough testing |

## Success Criteria

### MVP (Minimum Viable Product)
- [ ] All core domain models implemented
- [ ] Database migrations working
- [ ] API endpoints for CRUD operations on all main entities
- [ ] Authentication working
- [ ] Basic WebSocket functionality
- [ ] Background jobs working
- [ ] Contract tests passing for all critical paths

### Full Feature Parity
- [ ] All API endpoints implemented
- [ ] All background jobs implemented
- [ ] All WebSocket features implemented
- [ ] All external service integrations working
- [ ] All admin features implemented
- [ ] Performance at least as good as Ruby version
- [ ] 100% contract test coverage

### Production Ready
- [ ] Comprehensive monitoring
- [ ] Documentation complete
- [ ] Security audit passed
- [ ] Performance benchmarks acceptable
- [ ] Deployment pipeline working
- [ ] Rollback plan in place

## Next Steps

1. **Review and approve this plan**
2. **Set up Rust project structure** (Phase 0)
3. **Begin domain layer implementation** (Phase 1)
4. **Set up CI/CD pipeline**
5. **Start weekly sync meetings**

## Appendix

### A. Model Count Summary

| Category | Count | Complexity |
|----------|-------|------------|
| Models | 38 | High |
| Controllers | 203 | Medium |
| Services | ~50 | Medium |
| Background Jobs | ~10 | Medium |
| WebSocket Handlers | ~5 | Low |

### B. External Dependencies

- **Search Engines**: Solr, OpenSearch, Elasticsearch
- **LLM Providers**: OpenAI, Anthropic, Google, Ollama
- **Authentication**: Keycloak, Google, OpenID Connect
- **Email**: Postmark
- **Database**: MySQL, PostgreSQL, SQLite
- **Message Queue**: Redis (for jobs)

### C. Performance Goals

| Metric | Current (Ruby) | Target (Rust) | Improvement |
|--------|----------------|---------------|-------------|
| API Response Time (p95) | ~200ms | <100ms | 2x faster |
| Database Query Time | ~50ms | <30ms | 1.67x faster |
| Memory Usage | ~500MB | <300MB | 40% reduction |
| CPU Usage | ~2 cores | <1.5 cores | 25% reduction |
| Concurrent Users | ~1000 | >5000 | 5x improvement |
| Requests/Second | ~100 | >500 | 5x improvement |

### D. File Count Estimates

| Crate | Estimated Files | Estimated LOC |
|-------|-----------------|---------------|
| quepid-core | ~100 | ~10,000 |
| quepid-infra | ~80 | ~8,000 |
| quepid-api | ~150 | ~15,000 |
| quepid-workers | ~30 | ~3,000 |
| quepid-websocket | ~20 | ~2,000 |
| quepid-cli | ~10 | ~1,000 |
| **Total** | **~400** | **~40,000** |

Note: This compares to the current Ruby codebase of ~200 models/controllers + ~50K LOC in app/

### E. Glossary

- **Case**: Central entity for search testing, contains queries and configurations
- **Query**: A search term with associated ratings and results
- **Rating**: User rating of a search result for a query
- **Score**: Aggregated score for a case based on ratings
- **Scorer**: Algorithm for calculating scores from ratings
- **Try**: A configuration change snapshot for A/B testing
- **Book**: Collection of queries and results for offline judging
- **Judgement**: User judgement of a query-document pair in a book
- **Snapshot**: Point-in-time snapshot of case state

---

*Document Version: 1.0*
*Last Updated: $(date)*
*Author: Vibe Code (Mistral AI)*
