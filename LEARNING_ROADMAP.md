# Rust Learning Roadmap: From Scala to Production AI Server

**Project**: Rust (Claude Projects)  
**Target Audience**: Scala developer with strong concurrency knowledge  
**Goal**: Build a concurrent server handling AI workloads  
**Timeline**: 4-6 weeks (adjustable based on time commitment)

---

## 📋 How to Use This Roadmap

This document is your structured learning path. Use the checkboxes to track progress as you complete each section. Update this file regularly to maintain momentum and reflect on what you've learned.

---

## Table of Contents
1. [Prerequisites & Setup](#phase-0-prerequisites--setup)
2. [Phase 1: Rust Fundamentals](#phase-1-rust-fundamentals-week-1)
3. [Phase 2: Ownership Deep Dive](#phase-2-ownership-deep-dive-week-1-2)
4. [Phase 3: Concurrency Foundations](#phase-3-concurrency-foundations-week-2-3)
5. [Phase 4: Async Runtime & Tokio](#phase-4-async-runtime--tokio-week-3-4)
6. [Phase 5: Web Server Basics](#phase-5-web-server-basics-week-4-5)
7. [Phase 6: AI Workload Integration](#phase-6-ai-workload-integration-week-5-6)
8. [Resources](#resources)
9. [Progress Tracker](#progress-tracker)

---

## Phase 0: Prerequisites & Setup

### Environment Setup
- [ ] Install Rust via rustup: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- [ ] Verify installation: `rustc --version` and `cargo --version`
- [ ] Install VS Code with rust-analyzer extension (or your preferred IDE)
- [ ] Create a GitHub repo for your learning projects
- [ ] Install clippy and rustfmt: `rustup component add clippy rustfmt`

### Mindset Shifts from Scala
| Scala Concept | Rust Equivalent | Key Difference |
|---------------|-----------------|----------------|
| JVM GC | Ownership system | No GC - deterministic memory management |
| Implicit parameters | Traits + generics | Explicit, compile-time resolution |
| Future/ExecutionContext | Future/Runtime (Tokio) | Explicit runtime, zero-cost abstractions |
| Case classes | Structs + Enums | Enums are algebraic types with data |
| Option/Either | Option/Result | Result for error handling is idiomatic |
| Thread pools | Tokio runtime | Async-first, work-stealing scheduler |

---

## Phase 1: Rust Fundamentals (Week 1)

### 1.1 Basic Syntax & Types (Days 1-2)
**Goal**: Understand Rust's type system and basic syntax

#### Tasks:
- [ ] Complete Rust Book Chapters 1-3
- [ ] Write a simple CLI tool (e.g., file word counter)
- [ ] Practice pattern matching with enums

#### Key Concepts:
- Scalar types: integers, floats, booleans, chars
- Compound types: tuples, arrays
- String vs &str (crucial difference!)
- Enums with data (like Scala's sealed traits + case classes combined)

#### Mini Project: CLI Calculator
```rust
// Goals:
// - Parse command line arguments
// - Pattern match on operations
// - Handle errors with Result<T, E>
```

**Scala Parallels**:
- `Vec<T>` ≈ `List[T]` (but mutable by default)
- `Option<T>` ≈ `Option[T]`
- `Result<T, E>` ≈ `Either[E, T]` (but reversed, success first)

---

### 1.2 Functions & Control Flow (Days 3-4)
**Goal**: Master Rust's expression-based syntax

#### Tasks:
- [ ] Understand expression vs statement
- [ ] Practice with if/else, loops, iterators
- [ ] Write higher-order functions

#### Key Concepts:
- Everything is an expression (like Scala!)
- No early return needed (implicit return)
- Iterator combinators: map, filter, fold, etc.

#### Mini Project: JSON Parser (Simple)
```rust
// Goals:
// - Use iterators extensively
// - Pattern matching on recursive structures
// - Error propagation with ?
```

---

## Phase 2: Ownership Deep Dive (Week 1-2)

### 2.1 Ownership & Borrowing (Days 5-7)
**Goal**: Internalize Rust's memory model (THE crucial difference from Scala)

#### Tasks:
- [ ] Complete Rust Book Chapter 4 thoroughly
- [ ] Experiment with borrowing rules
- [ ] Understand when moves happen vs copies

#### Key Concepts:
- Ownership rules (one owner, borrowing, lifetimes)
- Mutable vs immutable borrows
- Copy vs Move semantics
- Lifetimes (elision rules first, explicit later)

#### Mental Model:
```
Scala:                          Rust:
val x = Data()                  let x = Data::new();
val y = x  // shallow copy       let y = x;  // MOVED! x is invalid now
x.method() // still works        // x.method() // COMPILE ERROR

val x = Data()                  let x = Data::new();
val y = x                       let y = &x;  // borrowed
x.method() // works             x.method(); // works, x still owns
```

#### Mini Project: In-Memory Key-Value Store
```rust
// Goals:
// - Manage lifetimes explicitly
// - Use Rc<T> and RefCell<T> when needed
// - Understand interior mutability
```

---

### 2.2 Lifetimes & Advanced Borrowing (Days 8-10)
**Goal**: Handle complex borrowing scenarios

#### Tasks:
- [ ] Understand lifetime annotations
- [ ] Practice with struct lifetimes
- [ ] Learn when to use `'static`

#### Key Concepts:
- Lifetime elision rules
- Struct lifetime parameters
- `'static` lifetime
- Multiple lifetime parameters

#### Mini Project: Text Buffer with Slices
```rust
// Goals:
// - Return references with explicit lifetimes
// - Create zero-copy string processing
// - Understand string slices deeply
```

---

## Phase 3: Concurrency Foundations (Week 2-3)

### 3.1 Threads & Message Passing (Days 11-13)
**Goal**: Understand OS-level threading in Rust

#### Tasks:
- [ ] Complete Rust Book Chapter 16
- [ ] Compare to Scala's Thread/ExecutionContext
- [ ] Build a thread pool from scratch

#### Key Concepts:
- `std::thread::spawn`
- Channels: `mpsc::channel()` (like Akka actors messaging)
- Arc<T> for shared ownership across threads
- Mutex<T> and RwLock<T> for shared state

#### Scala Comparison:
```scala
// Scala
implicit val ec: ExecutionContext = ...
Future { heavyComputation() }

// Rust (thread-based)
use std::thread;
thread::spawn(|| {
    heavy_computation()
});
```

#### Mini Project: Parallel File Processor
```rust
// Goals:
// - Spawn worker threads
// - Use channels for work distribution
// - Aggregate results with Arc<Mutex<T>>
```

---

### 3.2 Shared State & Synchronization (Days 14-16)
**Goal**: Master thread-safe shared state

#### Tasks:
- [ ] Understand Arc vs Rc
- [ ] Practice with Mutex and RwLock
- [ ] Learn about atomic types
- [ ] Compare performance characteristics

#### Key Concepts:
- Arc<Mutex<T>> pattern (shared mutable state)
- Arc<RwLock<T>> for read-heavy workloads
- Atomic types: AtomicUsize, AtomicBool, etc.
- Send and Sync traits (compiler-enforced thread safety!)

#### Mini Project: Thread-Safe Counter & Metrics
```rust
// Goals:
// - Build a metrics collector
// - Multiple threads increment counters
// - Periodic reporter thread
// - Compare Mutex vs Atomic performance
```

**Key Insight**: Rust's type system prevents data races at compile time!
- `Send`: can transfer ownership between threads
- `Sync`: can share references between threads

---

## Phase 4: Async Runtime & Tokio (Week 3-4)

### 4.1 Async/Await Fundamentals (Days 17-19)
**Goal**: Transition from thread-based to async concurrency

#### Tasks:
- [ ] Understand Future trait
- [ ] Learn async/await syntax
- [ ] Compare to Scala's Future/Promise
- [ ] Install and explore Tokio

#### Key Concepts:
- `async fn` syntax
- `.await` for suspending execution
- Futures are lazy (like Scala's Future with execution context)
- Runtimes execute futures (Tokio ≈ ExecutionContext)

#### Scala vs Rust Async:
```scala
// Scala
def fetchData(): Future[String] = Future {
  // runs immediately on ec
  httpGet("url")
}

// Rust
async fn fetch_data() -> String {
  // Returns Future, doesn't run yet!
  http_get("url").await
}

// Must spawn or await to run:
tokio::spawn(fetch_data());
```

#### Mini Project: Async File Downloader
```rust
// Goals:
// - Use tokio::fs for async file I/O
// - Concurrent downloads with join!
// - Error handling with Result
```

---

### 4.2 Tokio Runtime Deep Dive (Days 20-22)
**Goal**: Master Tokio's work-stealing scheduler

#### Tasks:
- [ ] Understand multi-threaded vs current-thread runtime
- [ ] Learn task spawning and joining
- [ ] Practice with select! macro
- [ ] Explore channels: mpsc, oneshot, broadcast

#### Key Concepts:
- `tokio::spawn` (spawn task on runtime)
- `tokio::join!` (concurrent await)
- `tokio::select!` (like Scala's Future.firstCompletedOf)
- Async channels vs std channels

#### Architecture Understanding:
```
Tokio Runtime Architecture:
┌─────────────────────────────────┐
│   Work-Stealing Scheduler       │
│  ┌─────┐  ┌─────┐  ┌─────┐     │
│  │ Core│  │ Core│  │ Core│     │
│  │Queue│  │Queue│  │Queue│     │
│  └─────┘  └─────┘  └─────┘     │
│     ↓         ↓         ↓       │
│  [Tasks] [Tasks] [Tasks]        │
└─────────────────────────────────┘

Compare to Scala's ExecutionContext:
- Work stealing (vs thread pool)
- Green threads (vs OS threads)
- Explicit runtime (vs implicit)
```

#### Mini Project: Chat Server (No HTTP Yet)
```rust
// Goals:
// - TCP listener with Tokio
// - Broadcast messages to all clients
// - Handle client disconnections
// - Use Arc<Mutex<HashMap>> for client registry
```

---

## Phase 5: Web Server Basics (Week 4-5)

### 5.1 HTTP with Axum (Days 23-26)
**Goal**: Build REST APIs with Axum framework

#### Tasks:
- [ ] Install Axum and understand routing
- [ ] Learn extractors (Path, Query, Json)
- [ ] Implement middleware
- [ ] Add state management

#### Key Concepts:
- Router and handlers
- Extractors pattern
- Shared state with Arc
- Error handling with custom types

#### Scala Comparison:
```scala
// Akka HTTP / Play
val route = 
  path("users" / IntNumber) { id =>
    get { complete(getUser(id)) }
  }

// Axum
async fn get_user(Path(id): Path<i32>) -> Json<User> {
    Json(fetch_user(id).await)
}

let app = Router::new()
    .route("/users/:id", get(get_user));
```

#### Mini Project: REST API for Task Manager
```rust
// Goals:
// - CRUD operations
// - JSON serialization with serde
// - In-memory storage (Arc<RwLock<HashMap>>)
// - Error responses
```

---

### 5.2 Middleware & State (Days 27-29)
**Goal**: Add cross-cutting concerns and shared state

#### Tasks:
- [ ] Create custom middleware
- [ ] Add request logging
- [ ] Implement authentication middleware
- [ ] Manage application state

#### Mini Project: Enhanced API with Auth
```rust
// Goals:
// - JWT authentication middleware
// - Rate limiting per user
// - Request/response logging
// - Metrics collection
```

---

## Phase 6: AI Workload Integration (Week 5-6)

### 6.1 CPU-Bound Task Handling (Days 30-33)
**Goal**: Integrate AI model inference without blocking async runtime

#### Tasks:
- [ ] Understand spawn_blocking for CPU-bound work
- [ ] Create a task queue system
- [ ] Implement backpressure
- [ ] Add worker pool for AI tasks

#### Key Concepts:
- `tokio::task::spawn_blocking` (offload to thread pool)
- Rayon for data parallelism
- Channel-based task distribution
- Semaphore for concurrency limits

#### Architecture Pattern:
```rust
// AI Workload Server Pattern
┌──────────────────────────────┐
│      Axum HTTP Server        │
│    (Async I/O - Tokio)       │
└──────────┬───────────────────┘
           │
           │ Request received
           ↓
    ┌─────────────┐
    │ Task Queue  │
    │  (Channel)  │
    └──────┬──────┘
           │
           ↓
  ┌─────────────────────┐
  │  Worker Thread Pool │
  │  (spawn_blocking)   │
  │  ┌────┐  ┌────┐    │
  │  │CPU │  │CPU │    │
  │  │Task│  │Task│    │
  │  └────┘  └────┘    │
  └─────────────────────┘
           │
           ↓
    Result back to client
```

#### Mini Project: Image Processing Service
```rust
// Goals:
// - Accept image uploads
// - Process in background thread pool
// - Return job ID immediately
// - Poll for completion
// - Use spawn_blocking for CPU work
```

---

### 6.2 Real AI Integration (Days 34-38)
**Goal**: Build production-ready AI inference server

#### Tasks:
- [ ] Integrate a Rust ML library (candle, tract, or burn)
- [ ] Load and cache model weights
- [ ] Implement batching for efficiency
- [ ] Add request/response streaming
- [ ] Monitor performance metrics

#### Technology Options:
1. **Candle**: Rust ML framework by Hugging Face
2. **Tract**: ONNX runtime in Rust
3. **Burn**: Deep learning framework
4. **FFI to Python**: Using PyO3 (if needed)

#### Final Project: AI Inference Server
```rust
// Goals:
// - Load pretrained model at startup
// - REST endpoint for inference
// - Batch requests for efficiency
// - Stream responses for large outputs
// - Concurrent request handling
// - CPU/GPU utilization metrics
// - Request queuing with backpressure

// Example endpoints:
// POST /v1/inference - Submit inference job
// GET /v1/jobs/:id - Check job status
// GET /v1/stream/inference - Server-sent events for streaming
```

#### Advanced Features:
- [ ] Request batching (collect N requests, infer together)
- [ ] Model warmup at startup
- [ ] Graceful shutdown (finish in-flight requests)
- [ ] Health check endpoint
- [ ] Prometheus metrics export
- [ ] Dynamic worker scaling

---

## Resources

### Books & Documentation
- [ ] **The Rust Book** (official) - Chapters 1-16, 20
- [ ] **Rust by Example** - For hands-on practice
- [ ] **Tokio Tutorial** - Official async runtime guide
- [ ] **Axum Documentation** - Web framework guide

### Comparisons for Scala Developers
- **Ownership** ≈ Unique references + linear types (but compile-time enforced)
- **Traits** ≈ Type classes (more powerful, with associated types)
- **Async/await** ≈ Future/Promise (but lazy by default)
- **Match** ≈ Pattern matching (exhaustiveness checked)
- **Cargo** ≈ SBT (but faster and simpler)

### Useful Crates
| Purpose | Crate | Notes |
|---------|-------|-------|
| Async runtime | `tokio` | Like ExecutionContext + event loop |
| Web framework | `axum` | Type-safe, fast, built on Tower |
| Serialization | `serde` | Like circe/play-json |
| Error handling | `anyhow`, `thiserror` | Better than just Result |
| Logging | `tracing` | Structured logging |
| Testing | `criterion` | Benchmarking |
| AI/ML | `candle`, `tract` | ML frameworks |
| Parallel processing | `rayon` | Data parallelism |

---

## Progress Tracker

### Week 1: Fundamentals
- [ ] Day 1-2: Basic syntax, types, control flow
- [ ] Day 3-4: Functions, iterators, error handling
- [ ] Day 5-7: Ownership, borrowing, moves
- **Checkpoint**: Build CLI tool with error handling

### Week 2: Advanced Ownership & Concurrency
- [ ] Day 8-10: Lifetimes, advanced borrowing
- [ ] Day 11-13: Threads, channels, Arc/Mutex
- [ ] Day 14-16: Synchronization primitives
- **Checkpoint**: Build parallel file processor

### Week 3-4: Async & Web
- [ ] Day 17-19: Async fundamentals, Tokio basics
- [ ] Day 20-22: Tokio runtime, channels, select
- [ ] Day 23-26: Axum, routing, extractors
- [ ] Day 27-29: Middleware, state, auth
- **Checkpoint**: Build async REST API

### Week 5-6: AI Workloads
- [ ] Day 30-33: CPU-bound tasks, spawn_blocking
- [ ] Day 34-38: ML integration, inference server
- **Final Checkpoint**: Production AI inference server

---

## Daily Learning Routine

### Recommended 2-hour daily schedule:
- **30 min**: Read documentation/book chapter
- **60 min**: Hands-on coding (mini projects)
- **30 min**: Experiment and break things

### Weekly Milestones:
Each week should produce a working project demonstrating that week's concepts.

---

## Key Gotchas for Scala Developers

1. **Strings**: `String` is owned, `&str` is borrowed. Always a pain point initially.
2. **No null**: Use `Option<T>` everywhere (you know this!)
3. **No inheritance**: Use composition + traits instead
4. **Explicit errors**: `Result<T, E>` is preferred over exceptions
5. **No implicit conversions**: Use `From`/`Into` traits explicitly
6. **Clone is explicit**: No hidden copying like JVM
7. **Macros are different**: More powerful but different syntax than Scala macros
8. **Async is lazy**: Must explicitly run (spawn or await)

---

## Success Metrics

By the end of this plan, you should be able to:
- [ ] Explain ownership and borrowing to others
- [ ] Write memory-safe concurrent code confidently
- [ ] Build async web services with Tokio/Axum
- [ ] Integrate CPU-bound workloads without blocking I/O
- [ ] Deploy a production-grade AI inference server
- [ ] Read and understand most Rust codebases
- [ ] Contribute to Rust open-source projects

---

## Next Steps After Completion

- **Advanced Topics**: Unsafe Rust, custom allocators, proc macros
- **Production**: Deployment, monitoring, profiling
- **Ecosystem**: Explore domain-specific crates
- **Community**: Contribute to open source, join Rust forums

---

**Last Updated**: February 2026
**Maintainer**: Your learning journey!

Good luck! Remember: fighting the borrow checker is part of learning. Those compiler errors are teaching you! 🦀
