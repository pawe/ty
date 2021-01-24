FROM rust:1.49 as build
# sqlx uses the 'sqlx-data.json' for compile time checks
ENV SQLX_OFFLINE true
COPY ./ ./

RUN cargo build -p ty-server --release
RUN mkdir -p /build-out
RUN cp target/release/ty-server /build-out/

RUN cargo install wasm-pack 
RUN wasm-pack build --target web --out-name wasm --out-dir /build-out/static --release ty-spa
RUN cp ty-spa/static/index.html /build-out/static/

FROM debian:buster-slim AS tyserver

COPY --from=build /build-out/ty-server /
COPY --from=build /build-out/static/ /static/
CMD /ty-server