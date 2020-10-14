FROM rust:1.47.0 as build
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /app

COPY . /app

RUN cargo build --release

FROM gcr.io/distroless/cc

COPY --from=build /app/target/release/vader_sentiment_api /

CMD ["./vader_sentiment_api"]