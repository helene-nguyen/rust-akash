# ============================================
# Stage 1: Chef (dependency caching)
# ============================================
FROM --platform=$BUILDPLATFORM rust:1.88-bookworm AS chef
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

ARG TARGETPLATFORM

RUN case "$TARGETPLATFORM" in \
      "linux/amd64") echo "x86_64-unknown-linux-gnu" > /tmp/target ;; \
      "linux/arm64") echo "aarch64-unknown-linux-gnu" > /tmp/target ;; \
      *) echo "Unsupported: $TARGETPLATFORM" && exit 1 ;; \
    esac \
    && rustup target add $(cat /tmp/target)

RUN dpkg --add-architecture arm64 \
    && apt-get update && apt-get install -y --no-install-recommends \
       gcc-aarch64-linux-gnu \
       libc6-dev-arm64-cross \
    && rm -rf /var/lib/apt/lists/*

ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --target $(cat /tmp/target) --recipe-path recipe.json

COPY . .
RUN cargo build --release --target $(cat /tmp/target) \
    && mkdir -p /app/out \
    && cp target/$(cat /tmp/target)/release/akash /app/out/ \
    && cp target/$(cat /tmp/target)/release/aka /app/out/

# ============================================
# Stage 4: Runtime
# ============================================
FROM gcr.io/distroless/cc-debian12

COPY --from=builder /app/out/akash /usr/local/bin/akash
COPY --from=builder /app/out/aka /usr/local/bin/aka

ENTRYPOINT ["akash"]