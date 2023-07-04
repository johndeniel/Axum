# Rust Axum 

<div align="justify"> This repository is dedicated to showcasing the power and versatility of the Axum framework in Rust programming language. Axum is a high-performance web framework designed for building scalable and asynchronous applications. </div>

## Repository Contents

- **section_01**: axum-api
- **section_02**: post-body
- **section_03**: extract-json
- **section_04**: path-variable
- **section_05**: query-params
- **section_06**: std-header
- **section_07**: custom-header
- **section_08**: cors-middleware
- **section_09**: shared-middleware
- **section_10**: custom-middleware

## Getting Started

```toml
# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
axum = { version = "0.6.18", features = ["headers"] }
serde = { version = "1.0.164", features = ["derive"] }
tokio = { version = "1.28.2", features = ["macros", "rt-multi-thread"] }
tower-http = { version = "0.4.1", features = ["cors"] }
```