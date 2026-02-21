# ============================================
# Stage 1: Chef (dependency caching)
# ============================================
FROM rust:1.88-bookworm AS chef
RUN cargo install cargo-chef
WORKDIR /app

# ============================================
# Stage 2: Plan dependencies
# ============================================
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# ============================================
# Stage 3: Build
# ============================================
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release \
    && strip target/release/akash target/release/aka

# ============================================
# Stage 4: Runtime
# ============================================
FROM gcr.io/distroless/cc-debian12

COPY --from=builder /app/target/release/akash /usr/local/bin/akash
COPY --from=builder /app/target/release/aka /usr/local/bin/aka

ENTRYPOINT ["akash"]