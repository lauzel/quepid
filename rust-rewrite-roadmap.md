# Quepid Rust Rewrite - Implementation Roadmap

## Quick Reference

This is a condensed version of the full plan. See [RUST_REWRITE_PLAN.md](./RUST_REWRITE_PLAN.md) for complete details.

## Overview

**Goal**: Rewrite Quepid from Ruby on Rails to Rust with:
- Full feature parity
- Improved performance (2-5x faster)
- Better type safety
- Automated testing against existing Ruby API
- Clean Architecture + Hexagonal pattern

**Estimated Duration**: 22 weeks (5.5 months)

**Technology Stack**:
- **Framework**: Axum
- **ORM**: SeaORM
- **Database**: MySQL/PostgreSQL/SQLite (same as current)
- **Auth**: JWT + OAuth2/OIDC
- **Background Jobs**: Custom worker pool + Redis
- **WebSockets**: Tokio + tungstenite
- **API Docs**: OpenAPI (utoipa)
- **Testing**: Rust test + mockall + testcontainers + contract testing

## Phase Breakdown

### 🏗️ Phase 0: Foundation (Week 1-2)
Set up project structure and tooling

```
quepid-rust/
├── Cargo.toml                    # Workspace
├── crates/
│   ├── quepid-core/              # Domain layer
│   ├── quepid-infra/             # Infrastructure
│   ├── quepid-api/               # HTTP API
│   ├── quepid-workers/           # Background jobs
│   └── quepid-websocket/         # WebSocket server
└── docker/
    ├── Dockerfile
    └── docker-compose.yml
```

**Tasks**:
- [ ] Create workspace structure
- [ ] Set up CI/CD (GitHub Actions)
- [ ] Configure Docker for development
- [ ] Extract OpenAPI spec from Rails
- [ ] Set up linting (clippy, rustfmt)

**Deliverables**:
- Working Rust project
- CI pipeline
- Development environment

---

### 🧱 Phase 1: Domain Layer (Week 3-4)
Implement core business logic

**Entities to implement** (38 total):
- User, Team, Case, Query, Rating, Score
- Scorer, Try, SearchEndpoint, Snapshot
- Book, QueryDocPair, Judgement
- Annotation, ApiKey, Announcement, etc.

**Tasks**:
- [ ] Define all domain entities as Rust structs
- [ ] Implement value objects (EmailAddress, etc.)
- [ ] Create domain services (RatingCalculator, etc.)
- [ ] Define custom error types
- [ ] Write comprehensive unit tests

**Deliverables**:
- `quepid-core` crate
- Domain test suite

---

### 🗄️ Phase 2: Infrastructure Layer (Week 5-8)
Database, external services, background jobs

**Database (SeaORM)**:
- [ ] Define all SeaORM entities
- [ ] Implement repository pattern
- [ ] Create database migrations
- [ ] Support MySQL, PostgreSQL, SQLite

**External Services**:
- [ ] Search engine adapters (Solr, OpenSearch)
- [ ] LLM judge adapters (OpenAI, Anthropic, etc.)
- [ ] Email service (Postmark)
- [ ] Analytics

**Background Jobs**:
- [ ] Worker pool implementation
- [ ] Redis-based job queue
- [ ] Job types: ExportCase, ImportCase, RunAiJudge, etc.

**WebSockets**:
- [ ] WebSocket server
- [ ] Connection management
- [ ] Message routing

**Deliverables**:
- `quepid-infra` crate
- Database migrations
- Background job system

---

### 🎯 Phase 2.5: Testing Framework (Parallel with Phase 2-3)
Build automated comparison testing

**Key Component**: Contract testing framework that:
1. Captures responses from Ruby API
2. Compares with Rust API responses
3. Flags discrepancies for investigation

```rust
// Example test
#[tokio::test]
async fn test_case_list_matches_ruby() {
    let ruby_response = get_ruby_response("/api/v1/cases");
    let rust_response = get_rust_response("/api/v1/cases");
    
    // Flexible comparison that ignores:
    // - Timestamps (compare as dates)
    // - Newly created IDs
    // - Non-deterministic array order
    assert_responses_match(&ruby_response, &rust_response);
}
```

**Tasks**:
- [ ] Build contract testing framework
- [ ] Record Ruby API baselines
- [ ] Implement flexible comparison logic
- [ ] Integrate with CI
- [ ] Set up performance benchmarking

**Deliverables**:
- Contract test suite
- Baseline recordings
- Performance benchmarks

---

### 🔧 Phase 3: Application Layer (Week 9-12)
Use cases and application services

**Authentication**:
- [ ] JWT token management
- [ ] Password hashing (bcrypt/argon2)
- [ ] Session management
- [ ] OAuth2/OIDC (Keycloak, Google, OpenID Connect)
- [ ] Authorization middleware

**Use Cases** (all API operations):
- [ ] User management (CRUD, auth)
- [ ] Case management
- [ ] Query management
- [ ] Rating management
- [ ] Score calculation
- [ ] Snapshot management
- [ ] Book management
- [ ] Judgement management
- [ ] Scorer management
- [ ] Search endpoint management
- [ ] Team management
- [ ] AI Judge management
- [ ] Admin operations

**Real-time**:
- [ ] WebSocket message handlers
- [ ] Progress notifications
- [ ] Job status updates

**Deliverables**:
- `quepid-api` crate with use cases
- Complete authentication system

---

### 🌐 Phase 4: Presentation Layer (Week 13-14)
HTTP API and WebSocket endpoints

**Tasks**:
- [ ] Define all routes (match routes.rb)
- [ ] Request validation
- [ ] Response formatting
- [ ] Error handling middleware
- [ ] Logging middleware
- [ ] Rate limiting
- [ ] CORS configuration
- [ ] Health checks

**API Endpoints to implement** (~100+):
- Auth: /users/login, /users/signup
- Cases: /api/v1/cases, /cases
- Queries: /api/v1/queries
- Ratings: /api/v1/ratings
- Scores: /api/v1/scores
- Books: /api/v1/books
- Judgements: /api/v1/judgements
- Scorers: /api/v1/scorers
- Search endpoints: /api/v1/search_endpoints
- AI Judges: /api/v1/ai_judges
- Teams: /api/v1/teams
- Admin: /admin/*
- Proxy: /proxy/fetch
- Dropdowns: /dropdown/*
- Mapper wizard: /search_endpoints/mapper_wizard/*
- Bulk judge: /bulk_judge/*

**Deliverables**:
- Complete HTTP API server
- WebSocket server integration

---

### ✅ Phase 5: Testing & Validation (Week 15-16)
Ensure Rust matches Ruby behavior

**Testing Strategy**:
1. **Unit Tests**: Domain logic, services
2. **Integration Tests**: API endpoints with real database
3. **Contract Tests**: Compare Rust vs Ruby responses
4. **Property-Based Tests**: Domain logic invariants
5. **Performance Tests**: Benchmark vs Ruby

**Comparison Testing**:
```bash
# Record Ruby baseline
cargo test --test contract -- --record-baseline

# Compare Rust with baseline
cargo test --test contract -- --compare-baseline

# Run in CI
cargo test --all
```

**Tasks**:
- [ ] Run full contract test suite
- [ ] Investigate and fix discrepancies
- [ ] Performance benchmarking
- [ ] Load testing
- [ ] Security audit

**Deliverables**:
- All contract tests passing
- Performance benchmarks acceptable
- Security audit passed

---

### 🚀 Phase 6: Migration & Deployment (Week 17-18)
Deploy Rust alongside Ruby

**Deployment Strategy**:
1. Blue-green deployment
2. Database migration (zero downtime)
3. Feature flags for gradual rollout
4. Monitoring and alerting

**Tasks**:
- [ ] Set up blue-green deployment
- [ ] Create database migration scripts
- [ ] Migrate configuration
- [ ] Set up feature flags
- [ ] Configure monitoring
- [ ] Set up alerting
- [ ] Create rollback plan

**Monitoring**:
- Request rates, error rates, latencies
- Database query performance
- Background job metrics
- Distributed tracing

**Deliverables**:
- Production-ready deployment
- Monitoring and alerting
- Migration documentation

---

### 🎨 Phase 7: Frontend Integration (Week 19-20)
Update frontend to work with Rust API

**Tasks**:
- [ ] Update API client configuration
- [ ] Handle any response format differences
- [ ] Update WebSocket client
- [ ] Fix CORS issues
- [ ] Performance optimizations
- [ ] End-to-end testing
- [ ] Browser compatibility testing

**Deliverables**:
- Frontend working with Rust backend
- Updated tests

---

### 🎯 Phase 8: Cleanup & Optimization (Week 21-22)
Final polish

**Performance**:
- [ ] Database query optimization
- [ ] Caching implementation
- [ ] Connection pooling tuning
- [ ] Memory usage optimization
- [ ] CPU profiling

**Code Quality**:
- [ ] Clippy warnings cleanup
- [ ] Documentation completion
- [ ] Code coverage improvement
- [ ] Dependency updates

**Final Testing**:
- [ ] Security audit
- [ ] Penetration testing
- [ ] Chaos engineering tests

**Deliverables**:
- Optimized production code
- Complete documentation

---

## Timeline Summary

| Phase | Weeks | Focus |
|-------|-------|-------|
| 0 | 1-2 | Foundation |
| 1 | 3-4 | Domain |
| 2 | 5-8 | Infrastructure |
| 2.5 | 5-12 | Testing Framework (parallel) |
| 3 | 9-12 | Application |
| 4 | 13-14 | Presentation |
| 5 | 15-16 | Testing |
| 6 | 17-18 | Deployment |
| 7 | 19-20 | Frontend |
| 8 | 21-22 | Cleanup |

## Key Principles

### 1. **No Bug Reimplementation**
- If we find a bug in Ruby, fix it in Rust
- Don't replicate Ruby bugs in Rust
- Use contract tests to catch discrepancies

### 2. **Fully Automated Testing**
- CI runs all tests on every PR
- Contract tests compare Rust vs Ruby
- Performance tests catch regressions
- Property tests verify invariants

### 3. **Clean Architecture**
```
┌─────────────────────┐
│   Presentation       │  <- Axum, WebSocket
├─────────────────────┤
│   Application        │  <- Use Cases
├─────────────────────┤
│   Domain             │  <- Entities, Services
├─────────────────────┤
│   Infrastructure     │  <- Database, External Services
└─────────────────────┘
```

### 4. **Gradual Migration**
- Deploy Rust alongside Ruby
- Use feature flags for gradual rollout
- Monitor and compare behavior
- Roll back if issues found

## Quick Start Commands

```bash
# Build everything
cargo build --all --release

# Run linter
cargo clippy --all -- -D warnings

# Run formatter
cargo fmt --all

# Run tests
cargo test --all

# Run specific test
cargo test --package quepid-core --test domain_tests

# Run contract tests
cargo test --package quepid-api --test contract

# Run benchmarks
cargo bench --all

# Start API server
cargo run --package quepid-api --release

# Start worker
cargo run --package quepid-workers --release

# Start WebSocket server
cargo run --package quepid-websocket --release
```

## Success Metrics

### MVP (Week 8)
- [ ] Domain layer complete
- [ ] Infrastructure layer complete
- [ ] Basic API endpoints working
- [ ] Contract tests passing for core paths

### Beta (Week 16)
- [ ] All API endpoints implemented
- [ ] All background jobs working
- [ ] All WebSocket features working
- [ ] All contract tests passing
- [ ] Performance benchmarks acceptable

### Production (Week 22)
- [ ] All features implemented
- [ ] Security audit passed
- [ ] Deployment pipeline working
- [ ] Monitoring in place
- [ ] Documentation complete

---

## Resources

- [Full Plan](./RUST_REWRITE_PLAN.md)
- [Quepid Documentation](../docs/)
- [Developer Guide](../DEVELOPER_GUIDE.md)
- [App Structure](../docs/app_structure.md)
- [Data Mapping](../docs/data_mapping.md)

## Getting Started

1. Read the [full plan](./RUST_REWRITE_PLAN.md)
2. Review the [Quepid documentation](../docs/)
3. Start with Phase 0: Set up the project structure
4. Begin implementing the domain layer
5. Join the weekly sync meetings

---

*Last Updated: $(date)*
