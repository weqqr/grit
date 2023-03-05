# grit

build dependencies:
- rust
- npm
- protoc

runtime dependencies:
- postgresql

build:
- `cd web && npm run build && cd ..`
- `cargo build --release`

run:
- `cargo run --release`
