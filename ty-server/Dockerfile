FROM rust:1.49 as build
# sqlx uses the 'sqlx-data.json' for compile time checks
ENV SQLX_OFFLINE true
COPY ./ ./

RUN cargo build -p ty-server --release
RUN mkdir -p /build-out
RUN cp target/release/ty-server /build-out/

FROM debian:buster-slim AS tyserver

COPY --from=build /build-out/ty-server /
CMD /ty-server