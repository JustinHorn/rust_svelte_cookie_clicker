FROM rust

EXPOSE 8000


ADD src src
COPY Cargo.toml .
COPY db.sql .
RUN cargo build --release
CMD ["./target/release/rust_svelte_cookie_clicker"]