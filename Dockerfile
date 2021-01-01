FROM rust:1.49 as build

COPY ./ ./
RUN cargo build -p ty-server --release

RUN mkdir -p /build-out

RUN cp target/release/ty-server /build-out/

FROM debian:buster-slim

COPY --from=build /build-out/ty-server /
CMD /ty-server