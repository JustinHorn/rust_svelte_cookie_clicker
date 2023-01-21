FROM node:latest as frontend

WORKDIR /client
COPY client/package.json .
COPY client/svelte.config.js .
COPY client/tsconfig.json .
COPY client/.npmrc .
COPY client/vite.config.js .
ADD client/static static
ADD client/src src
RUN npm install
RUN npm run build


FROM rust as backend

ADD src src
COPY Cargo.toml .
RUN cargo build --release


FROM debian:buster-slim
EXPOSE 8000
COPY --from=frontend /client/build ./client/build
COPY --from=backend /target/release/rust_svelte_cookie_clicker .
COPY db.sql .
CMD ["./rust_svelte_cookie_clicker"]