ARG RUN_ID

# ---------- Stage 1: Build Rust ----------
FROM rust:1.90-bullseye AS builder
ARG RUN_ID

RUN apt-get update && apt-get install -y \
	libsqlite3-dev \
	pkg-config \
	libssl-dev

WORKDIR /app

COPY . .
RUN cargo build --release

# ---------- Stage 2: Build Tailwind ----------
FROM debian:bullseye-slim AS tailwind

RUN apt-get update && apt-get install -y curl ca-certificates \
	&& rm -rf /var/lib/apt/lists/*

# Install Tailwind standalone binary
RUN curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64 \
	&& chmod +x tailwindcss-linux-x64 \
	&& mv tailwindcss-linux-x64 /usr/local/bin/tailwindcss

WORKDIR /app

# Copy only what Tailwind needs (keeps cache efficient)
COPY tailwind.config.js .
COPY public/styles ./public/styles
COPY templates ./templates

# Build CSS
RUN tailwindcss \
	-i ./public/styles/main.css \
	-o ./public/styles/main.${RUN_ID}.css \
	--minify

# ---------- Stage 3: Runtime ----------
FROM debian:bullseye-slim
ARG RUN_ID

WORKDIR /app

RUN apt-get update && apt-get install -y \
	libsqlite3-0 \
	ca-certificates \
	libssl1.1 \
	&& apt-get clean

# Copy compiled binary and assets
COPY --from=builder /app/target/release/app ./app
COPY --from=builder /app/templates ./templates
COPY --from=builder /app/migrations ./migrations
COPY --from=builder /app/public ./public

# Copy Tailwind output
COPY --from=tailwind /app/public ./public

# Cache-busting rename logic
RUN for dir in public/styles public/scripts; do \
	if [ -d "$dir" ]; then \
	for file in "$dir"/*; do \
	[ -f "$file" ] || continue; \
	filename=$(basename -- "$file"); \
	name="${filename%.*}"; \
	ext="${filename##*.}"; \
	cp "$file" "$dir/$name.$RUN_ID.$ext"; \
	done; \
	fi; \
	done

CMD ["./app"]
